use crate::model::FunctionsToMap;
use crate::repository::output_repository::OutputRepository;
use std::sync::RwLock;

pub(crate) struct ExtractFunctionsService {
    output_repository: RwLock<Box<dyn OutputRepository>>,
}

impl ExtractFunctionsService {
    fn new(output_repository: RwLock<Box<dyn OutputRepository>>) -> Self {
        Self { output_repository }
    }

    pub(crate) fn run(&self) -> Vec<FunctionsToMap> {
        self.output_repository
            .read()
            .unwrap()
            .get_native_functions_to_map()
    }
}
