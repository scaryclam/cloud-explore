use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::fs::File;

use rustache::{HashBuilder, Render};
use rusoto_core::{Region};
use rusoto_ec2::{Vpc, Ec2, Ec2Client, DescribeVpcsRequest};


pub struct VPC {
    pub cidr_block: Option<String>,
    pub instance_tenancy: Option<String>,
    pub ipv_6_cidr_block_association: bool,
    pub tags: HashMap<Option<String>, Option<String>>,
    pub vpc_id: Option<String>
}


pub struct VPCExplorer {
    pub vpcs: Vec<VPC>
}

impl VPCExplorer {
    pub fn new() -> VPCExplorer {
        VPCExplorer {
            vpcs: Vec::new()
        }
    }

    fn build_vpc(&mut self, vpc: Vpc) {
        let mut tags = HashMap::new();
        match vpc.tags {
            Some(vpc_tags) => {
                for tag in vpc_tags {
                    tags.insert(tag.key, tag.value);
                }   
            }   
            None => ()
        }   
       
        let new_vpc = VPC {
            cidr_block: vpc.cidr_block,
            instance_tenancy: vpc.instance_tenancy,
            ipv_6_cidr_block_association: false,
            tags: tags,
            vpc_id: vpc.vpc_id
        };  
        
        self.vpcs.push(new_vpc);
    }

    pub fn explore(&mut self) {
        println!("Finding VPCs...");
        let client = Ec2Client::new(Region::EuWest1);
        let request = DescribeVpcsRequest::default();
        
        match client.describe_vpcs(request).sync() {
            Ok(output) => {
                match output.vpcs {
                    Some(vpc_list) => {
                        for vpc in vpc_list {
                            self.build_vpc(vpc);
                        }   
                    }   
                    None => println!("No VPCs!"),
                }   
            }   
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    pub fn list_vpcs(&self) -> std::io::Result<()> {
        let mut output_file = File::create("vpc.tf")?;
        for vpc in self.vpcs.iter() {
            let output = self.terraform_the_things(vpc);
            output_file.write_all(output.as_bytes());
        }
        Ok(())
    }

    //fn write_file(&self, output_file: std::result::Result<File, std::io::Error>, output: String) -> std::io::Result<()> {
    //        output_file.write_all(output);
    //        Ok(())
    //}

    pub fn terraform_the_things(&self, vpc: &VPC) -> std::string::String {
        let name = vpc.vpc_id.as_ref().unwrap().clone();
        let cidr_block = vpc.cidr_block.as_ref().unwrap().clone();
        let instance_tenancy = vpc.instance_tenancy.as_ref().unwrap().clone();
        let mut tags = String::new();

        for (key, value) in &vpc.tags {
            let tag_string = format!("{} = \"{}\"\n", key.clone().unwrap(), value.clone().unwrap());
            tags.push_str(&tag_string);
        }

        let data = HashBuilder::new().insert("name", name)
                                     .insert("cidr_block", cidr_block)
                                     .insert("instance_tenancy", instance_tenancy)
                                     .insert("tags", tags);

        let mut out = Cursor::new(Vec::new());
        let template = "
resource \"aws_vpc\" \"{{ name }}\" {
   cidr_block = \"{{ cidr_block }}\"
   instance_tenancy = \"{{ instance_tenancy }}\"
   tags = {
       {{{ tags }}}
   }
}\n\n";

        data.render(template, &mut out).unwrap();
        let output = String::from_utf8(out.into_inner()).unwrap();
        return output;
    }
}
