use anyhow::Context;
use anyhow::Result;

use crate::services::extract_fields_service::ExtractFieldsService;

struct Looper {
    extract_fields_service: ExtractFieldsService,
}

impl Looper {
    fn new(extract_fields_service: ExtractFieldsService) -> Self {
        Self {
            extract_fields_service,
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let mut changed = false;

            let extract_fields_service_changed = self
                .extract_fields_service
                .run()
                .context("extract_fields_service failed")?;

            changed = changed || extract_fields_service_changed;

            if !changed {
                break;
            }
        }

        Ok(())
    }
}
