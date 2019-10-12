extern crate rustache;
extern crate rusoto_core;
extern crate rusoto_ec2;

mod vpc_explorer;
mod subnet_explorer;

use vpc_explorer::VPCExplorer;
use subnet_explorer::SubnetExplorer;


pub struct ResourceExplorer {
    vpc_handler: VPCExplorer,
    subnet_explorer: SubnetExplorer
}


impl ResourceExplorer {
    pub fn new() -> ResourceExplorer {
        ResourceExplorer {
            vpc_handler: VPCExplorer::new(),
            subnet_explorer: SubnetExplorer::new()
        }
    }

    pub fn explore(&mut self) {
        self.vpc_handler.explore();
        self.vpc_handler.list_vpcs();
        self.subnet_explorer.explore();
        self.subnet_explorer.list_subnets();
    }
}


fn main() {
    println!("Started");
    let mut foo = ResourceExplorer::new();
    foo.explore();
}

