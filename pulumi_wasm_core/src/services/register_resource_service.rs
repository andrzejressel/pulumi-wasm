use crate::external::pulumi::Pulumi;
use crate::repository::output_repository::OutputRepository;
use std::sync::RwLock;

struct RegisterResourceService {
    output_repository: RwLock<Box<dyn OutputRepository>>,
    pulumi: Box<dyn Pulumi>,
}

impl RegisterResourceService {
    fn run(&self) -> bool {
        true
    }
}