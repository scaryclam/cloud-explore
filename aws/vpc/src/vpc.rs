pub struct VPC {
    cidr_block: Option<String>,
    instance_tenancy: Option<String>,
    ipv_6_cidr_block_association: bool,
    tags: HashMap<Option<String>, Option<String>>,
    vpc_id: Option<String>
}


pub struct VPCExplorer {
    vpcs: Vec<VPC>
}


pub impl VPCExplorer {
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

    pub fn list_vpcs(&self) {
        for vpc in self.vpcs.iter() {
            self.terraform_the_things(vpc);
        }   
    }

    pub fn terraform_the_things(&self, vpc: &VPC) {
        let name = vpc.vpc_id.as_ref().unwrap().clone();
        let cidr_block = vpc.cidr_block.as_ref().unwrap().clone();
        let instance_tenancy = vpc.instance_tenancy.as_ref().unwrap().clone();
        let tags = String::new();

        //for (key, value) in vpc.tags.clone() {
        //    println!("{}: \"{}\"", key.unwrap(), value.unwrap());
        //    tags.push_str(&key.as_ref().unwrap().clone());
        //}

        let data = HashBuilder::new().insert("name", name)
                                     .insert("cidr_block", cidr_block)
                                     .insert("instance_tenancy", instance_tenancy);

        let mut out = Cursor::new(Vec::new());
        let template = "
resource \"aws_vpc\" \"{{ name }}\" {
    cidr_block = \"{{ cidr_block }}\"
    instance_tenancy = \"{{ instance_tenancy }}\"
    tags = {
        Name = \"{{ name }}\"
    }\n
}\n\n";

        let result = data.render(template, &mut out).unwrap();
        println!("{}", String::from_utf8(out.into_inner()).unwrap());
        result;
    }

}

