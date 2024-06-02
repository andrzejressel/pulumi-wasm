use std::sync::RwLock;

use crate::model::{DoneOutput, OutputId};
use crate::repository::output_repository::OutputRepository;

struct SetFunctionsService {
    output_repository: RwLock<Box<dyn OutputRepository>>,
}

impl SetFunctionsService {
    fn new(output_repository: RwLock<Box<dyn OutputRepository>>) -> Self {
        Self { output_repository }
    }

    fn run(&self, results: Vec<(OutputId, DoneOutput)>) {
        let mut repo = self.output_repository.write().unwrap();

        results
            .into_iter()
            .for_each(|(output_id, output)| repo.add_done_output(output_id, output));
    }
}
