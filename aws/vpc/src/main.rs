extern crate rustache;
extern crate rusoto_core;
extern crate rusoto_ec2;

mod explorer;

use explorer::VPCExplorer;


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
    let mut foo = ResourceExplorer::new();
    foo.explore();
}

