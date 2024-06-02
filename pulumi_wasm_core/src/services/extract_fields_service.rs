use std::sync::RwLock;

use anyhow::{anyhow, Result};
use rmpv::{Utf8String, Value};

use crate::model::{DoneOutput, FieldsToExtract};
use crate::repository::output_repository::OutputRepository;

pub(crate) struct ExtractFieldsService {
    output_repository: RwLock<Box<dyn OutputRepository>>,
}

impl ExtractFieldsService {
    fn new(output_repository: RwLock<Box<dyn OutputRepository>>) -> Self {
        ExtractFieldsService { output_repository }
    }

    pub(crate) fn run(&self) -> Result<bool> {
        let fields_to_extract = {
            self.output_repository
                .read()
                .unwrap()
                .get_fields_to_extract()
        };
        let changed = !fields_to_extract.is_empty();

        for FieldsToExtract {
            id,
            field_name,
            output,
        } in fields_to_extract
        {
            log::info!(
                "Extracting field [{}] from output with id [{}] and content [{}]",
                field_name.to_string(),
                id.0,
                output
            );
            let key = Value::String(Utf8String::from(field_name.to_string()));
            let deps_key = Value::String(Utf8String::from(field_name.to_string() + "__deps"));
            match &output {
                Value::Map(map) => {
                    let maybe_value = map.iter().find(|(k, _)| k == &key).map(|(_, v)| v.clone());
                    let maybe_deps = map.iter().find(|(k, _)| k == &deps_key).map(|(_, v)| v.clone());

                    let value = match maybe_value {
                        None => return Err(anyhow!("Value is not present")),
                        Some(v) => v
                    };

                    let deps = match maybe_deps {
                        Some(Value::Array(deps)) => deps
                            .iter()
                            .map(|dep| match dep {
                                Value::String(string_dep) => Ok(string_dep.as_str().unwrap().to_string()),
                                _ => Err(anyhow!("Dep is not a string - it's [{}] instead", dep.clone())),
                            })
                            .collect::<Result<Vec<_>>>()?,
                        Some(deps) => return Err(anyhow!("List of deps is not an array - it's [{}] instead", deps)),
                        None => return Err(anyhow!("List of deps is not present")),
                    };

                    self.output_repository.write().unwrap().add_done_output(id, DoneOutput::new(value, deps));
                }
                _ => return Err(anyhow!("Field output with id [{}] relates to output which is not map - it's [{}] instead", id.0, output)),
            }
        }

        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use uuid::Uuid;

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
    fn should_return_error_when_output_does_not_exists() {
        let mut output_repository = MockOutputRepository::new();
        let output_id = OutputId::new(UUID_1);
        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Map(vec![]),
                )]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run().unwrap_err();
        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(vec!["Value is not present"], chain);
    }

    #[test]
    fn should_return_error_when_output_is_not_hashmap() {
        // let mut output_repository = OutputRepositoryImpl::new();
        let output_id = OutputId::new(UUID_1);
        let mut output_repository = MockOutputRepository::new();

        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Boolean(true),
                )]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run().unwrap_err();
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

    #[test]
    fn should_return_error_when_deps_does_not_exists() {
        let mut output_repository = MockOutputRepository::new();
        let output_id = OutputId::new(UUID_1);
        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Map(vec![(
                        Value::String(Utf8String::from("field_name".to_string())),
                        Value::Boolean(true),
                    )]),
                )]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run().unwrap_err();
        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(vec!["List of deps is not present"], chain);
    }

    #[test]
    fn should_return_error_when_deps_is_not_array() {
        let mut output_repository = MockOutputRepository::new();
        let output_id = OutputId::new(UUID_1);
        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Map(vec![
                        (
                            Value::String(Utf8String::from("field_name".to_string())),
                            Value::Boolean(true),
                        ),
                        (
                            Value::String(Utf8String::from("field_name__deps".to_string())),
                            Value::Boolean(true),
                        ),
                    ]),
                )]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run().unwrap_err();
        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(
            vec!["List of deps is not an array - it's [true] instead"],
            chain
        );
    }

    #[test]
    fn should_return_error_when_dep_is_not_string() {
        let mut output_repository = MockOutputRepository::new();
        let output_id = OutputId::new(UUID_1);
        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Map(vec![
                        (
                            Value::String(Utf8String::from("field_name".to_string())),
                            Value::Boolean(true),
                        ),
                        (
                            Value::String(Utf8String::from("field_name__deps".to_string())),
                            Value::Array(vec![Value::Boolean(true)]),
                        ),
                    ]),
                )]
            });

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let err = service.run().unwrap_err();
        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(vec!["Dep is not a string - it's [true] instead"], chain);
    }

    #[test]
    fn should_add_done_output() {
        let mut output_repository = MockOutputRepository::new();
        let output_id = OutputId::new(UUID_1);
        output_repository
            .expect_get_fields_to_extract()
            .returning(move || {
                vec![FieldsToExtract::new(
                    output_id,
                    FieldName::new("field_name".into()),
                    Value::Map(vec![
                        (
                            Value::String(Utf8String::from("field_name".to_string())),
                            Value::Boolean(true),
                        ),
                        (
                            Value::String(Utf8String::from("field_name__deps".to_string())),
                            Value::Array(vec![Value::String(Utf8String::from("dep"))]),
                        ),
                    ]),
                )]
            });
        output_repository
            .expect_add_done_output()
            .with(
                eq(output_id),
                eq(DoneOutput::new(Value::Boolean(true), vec!["dep".into()])),
            )
            .returning(|_, _| ());

        let service = ExtractFieldsService::new(RwLock::new(Box::new(output_repository)));

        let changed = service.run().unwrap();

        assert!(changed);
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
}
