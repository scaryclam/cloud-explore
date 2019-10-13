use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::fs::File;

use rustache::{HashBuilder, Render};
use rusoto_core::{Region};
use rusoto_ec2::{Subnet, Ec2, Ec2Client, DescribeSubnetsRequest};


pub struct Ec2Subnet {
    pub tags: HashMap<Option<String>, Option<String>>,
    pub subnet_id: Option<String>,
    pub vpc_id: Option<String>,
    pub availability_zone: Option<String>,
    pub availability_zone_id: Option<String>,
    pub cidr_block: Option<String>,
    pub map_public_ip_on_launch: Option<bool>,
    pub map_public_addess_on_creation: Option<bool>
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
            availability_zone: subnet.availability_zone,
            availability_zone_id: subnet.availability_zone_id,
            cidr_block: subnet.cidr_block,
            map_public_ip_on_launch: subnet.map_public_ip_on_launch,
            map_public_addess_on_creation: subnet.assign_ipv_6_address_on_creation
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

    pub fn list_subnets(&self) -> std::io::Result<()> {
        let mut output_file = File::create("subnet.tf")?;
        for subnet in self.subnets.iter() {
            let output = self.terraform_the_things(subnet);
            output_file.write_all(output.as_bytes());
        }   
        Ok(())
    }

    pub fn terraform_the_things(&self, subnet: &Ec2Subnet) -> std::string::String {
        let name = subnet.subnet_id.as_ref().unwrap().clone();
        let vpc_id = subnet.vpc_id.as_ref().unwrap().clone();
        let cidr_block = subnet.cidr_block.as_ref().unwrap().clone();
        let availability_zone = subnet.availability_zone.as_ref().unwrap().clone();
        let map_public_ip_on_launch = subnet.map_public_ip_on_launch.as_ref().unwrap().clone();
        let map_public_addess_on_creation = subnet.map_public_addess_on_creation.as_ref().unwrap().clone();
        let mut tags = String::new();

        for (key, value) in &subnet.tags {
            let tag_string = format!("{} = \"{}\"\n", key.clone().unwrap(), value.clone().unwrap());
            tags.push_str(&tag_string);
        }

        let data = HashBuilder::new().insert("name", name)
                                     .insert("vpc_id", vpc_id)
                                     .insert("cidr_block", cidr_block)
                                     .insert("availability_zone", availability_zone)
                                     .insert("map_public_ip_on_launch", map_public_ip_on_launch)
                                     .insert("map_public_addess_on_creation", map_public_addess_on_creation);

        let mut out = Cursor::new(Vec::new());
        let template = "
resource \"aws_subnet\" \"{{ name }}\" {
    vpc_id = {{ vpc_id }}
    cidr_block = {{ cidr_block }}
    availability_zone = {{ availability_zone }}
    map_public_ip_on_launch = {{ map_public_ip_on_launch }}
    map_public_addess_on_creation = {{ map_public_addess_on_creation }}

    tags = {
        {{{ tags }}}
    }
}\n\n";

        data.render(template, &mut out).unwrap();
        let output = String::from_utf8(out.into_inner()).unwrap();
        return output;
    }

}
