use std::collections::HashMap;
use std::io::Cursor;

use rustache::{HashBuilder, Render};
use rusoto_core::{Region};
use rusoto_ec2::{Subnet, Ec2, Ec2Client, DescribeSubnetsRequest};


pub struct Ec2Subnet {
    pub tags: HashMap<Option<String>, Option<String>>,
    pub subnet_id: Option<String>,
    pub vpc_id: Option<String>,
    pub cidr_block: Option<String>
}


pub struct SubnetExplorer {
    pub subnets: Vec<Ec2Subnet>
}


impl SubnetExplorer {
    pub fn new() -> SubnetExplorer {
        SubnetExplorer {
            subnets: Vec::new()
        }
    }

    fn build_subnet(&mut self, subnet: Subnet) {
        let mut tags = HashMap::new();
        match subnet.tags {
            Some(subnet_tags) => {
                for tag in subnet_tags {
                    tags.insert(tag.key, tag.value);
                }   
            }   
            None => ()
        }   
       
        let new_subnet = Ec2Subnet {
            tags: tags,
            vpc_id: subnet.vpc_id,
            subnet_id: subnet.subnet_id,
            cidr_block: subnet.cidr_block
        };  
        
        self.subnets.push(new_subnet);
    }

    pub fn explore(&mut self) {
        println!("Finding Subnets...");
        let client = Ec2Client::new(Region::EuWest1);
        let request = DescribeSubnetsRequest::default();
        
        match client.describe_subnets(request).sync() {
            Ok(output) => {
                match output.subnets {
                    Some(subnet_list) => {
                        for subnet in subnet_list {
                            self.build_subnet(subnet);
                        }   
                    }   
                    None => println!("No subnets!"),
                }   
            }   
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    pub fn list_subnets(&self) {
        for subnet in self.subnets.iter() {
            self.terraform_the_things(subnet);
        }   
    }

    pub fn terraform_the_things(&self, subnet: &Ec2Subnet) {
        let name = subnet.subnet_id.as_ref().unwrap().clone();
        let vpc_id = subnet.vpc_id.as_ref().unwrap().clone();
        let cidr_block = subnet.cidr_block.as_ref().unwrap().clone();
        let mut tags = String::new();

        for (key, value) in &subnet.tags {
            let tag_string = format!("{} = \"{}\"\n", key.clone().unwrap(), value.clone().unwrap());
            tags.push_str(&tag_string);
        }

        let data = HashBuilder::new().insert("name", name)
                                     .insert("vpc_id", vpc_id)
                                     .insert("cidr_block", cidr_block)
                                     .insert("tags", tags);

        let mut out = Cursor::new(Vec::new());
        let template = "
resource \"aws_subnet\" \"{{ name }}\" {
   vpc_id = {{ vpc_id }}
   cidr_block = {{ cidr_block }}
   tags = {
       {{{ tags }}}
   }
}\n\n";

        let result = data.render(template, &mut out).unwrap();
        println!("{}", String::from_utf8(out.into_inner()).unwrap());
        result;
    }

}
