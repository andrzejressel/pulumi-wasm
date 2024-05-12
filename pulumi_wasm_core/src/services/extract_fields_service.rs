use std::sync::RwLock;
use wit_bindgen_rt::cabi_realloc;
use crate::repository::output_repository::OutputRepository;
use anyhow::{anyhow, Result};
use log::error;
use rmpv::{Utf8String, Value};
use crate::model::{FieldName, FieldsToExtract};

struct ExtractFieldsService {
    output_repository: RwLock<Box<dyn OutputRepository>>,
}

impl ExtractFieldsService {
    fn new(output_repository: RwLock<Box<dyn OutputRepository>>) -> Self {
        ExtractFieldsService {
            output_repository,
        }
    }

    fn run(&self) -> Result<bool> {
        let fields_to_extract = {
            self.output_repository.read().unwrap().get_fields_to_extract()
        };
        let changed = !fields_to_extract.is_empty();

        for FieldsToExtract { id, field_name, output } in fields_to_extract {
            log::info!("Extracting field [{}] from output with id [{}] and content [{}]", field_name.to_string(), id.0, output);
            let key = Value::String(Utf8String::from(field_name.to_string()));
            let deps_key = Value::String(Utf8String::from(field_name.to_string() + "__deps"));
            match output {
                Value::Map(map) => {
                    let maybe_value = map.iter().find(|(k, _)| k == &key).map(|(_, v)| v.clone());
                    let maybe_deps = map.iter().find(|(k, _)| k == &deps_key).map(|(_, v)| v.clone());
                    
                    let deps = match maybe_deps {
                        Some(rmpv::Value::Array(deps)) => Some(deps),
                        _ => return Err(anyhow!("Field output with id [{}] relates to output which is not map"))
                        // None => None,
                        // Some(deps) => Some(protobuf_to_msgpack(deps).unwrap()),
                    };
                    
                }
                _ => return Err(anyhow!("Field output with id [{}] relates to output which is not map - it's [{}] instead", id.0, output)),
            }
        }

        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use msgpack_protobuf_converter::{protobuf_to_msgpack, Type};
    use crate::model::{FieldName, OutputId};
    use crate::repository::output_repository::MockOutputRepository;
    use crate::repository::output_repository_impl::OutputRepositoryImpl;
    use super::*;

    #[test]
    fn should_return_false_when_there_are_no_changes() {
        let output_repository = OutputRepositoryImpl::new();
        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));
        assert!(!service.run().unwrap());
    }

    #[test]
    fn should_return_error_when_output_is_not_hashmap() {
        // let mut output_repository = OutputRepositoryImpl::new();
        let output_id = OutputId::new(UUID_1);
        let mut output_repository = MockOutputRepository::new();

        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![
                    FieldsToExtract::new(output_id, FieldName::new("field_name".into()), Value::Boolean(true))
                ]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run()
            .unwrap_err();
        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(
            vec![
                "Field output with id [01010101-0101-0101-0101-010101010101] relates to output which is not map - it's [true] instead",
            ],
            chain
        );
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
}
