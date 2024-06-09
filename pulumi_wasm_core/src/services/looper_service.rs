use crate::model::FunctionsToMap;
use crate::repository::output_repository::OutputRepository;
use anyhow::Context;
use anyhow::Result;
use std::sync::RwLock;

use crate::services::extract_fields_service::ExtractFieldsService;
use crate::services::functions::extract_functions_service::ExtractFunctionsService;

struct Looper {
    extract_fields_service: ExtractFieldsService,
    output_repository: RwLock<Box<dyn OutputRepository>>,
    extract_functions_service: ExtractFunctionsService,
}

impl Looper {
    fn new(
        extract_fields_service: ExtractFieldsService,
        output_repository: RwLock<Box<dyn OutputRepository>>,
        extract_functions_service: ExtractFunctionsService,
    ) -> Self {
        Self {
            extract_fields_service,
            output_repository,
            extract_functions_service,
        }
    }

    fn run(self) -> Result<Vec<FunctionsToMap>> {
        loop {
            let extract_fields_service_result = self
                .extract_fields_service
                .run()
                .context("extract_fields_service failed")?;

            let output_repository_distribute_nothings_result = {
                self.output_repository
                    .write()
                    .unwrap()
                    .distribute_nothings()
            };

            if extract_fields_service_result || output_repository_distribute_nothings_result {
                break;
            }
        }

        Ok(self.extract_functions_service.run())
    }
}
