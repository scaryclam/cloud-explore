extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate rustache;
extern crate vpc;

use rustache::{HashBuilder, Render};

use vpc::vpc::{VPCExplorer};


pub struct ResourceExplorer {
    vpc_handler: VPCExplorer
}


impl ResourceExplorer {
    pub fn new() -> ResourceExplorer {
        ResourceExplorer {
            vpc_handler: VPCExplorer::new()
        }
    }

    pub fn explore(&mut self) {
        self.vpc_handler.explore();
        self.vpc_handler.list_vpcs();
    }
}


fn main() {
    println!("Started");
    let mut explorer = ResourceExplorer::new();
    explorer.explore();
}

