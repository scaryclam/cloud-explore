use std::collections::HashMap;


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

