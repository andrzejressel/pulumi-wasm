use std::collections::{BTreeSet, HashMap};

use crate::model::{
    CreateResourceOutput, DoneOutput, ExtractFieldOutput, FieldsToExtract, FunctionsToMap,
    NativeFunctionOutput, NothingOutput, OutputId,
};
use crate::repository::output_repository::OutputRepository;

#[derive(Debug, Default)]
pub(crate) struct OutputRepositoryImpl {
    // Ready values
    nothing_map: HashMap<OutputId, NothingOutput>,
    done_map: HashMap<OutputId, DoneOutput>,

    // Values waiting to be computed
    native_function_map: HashMap<OutputId, NativeFunctionOutput>,
    extract_fields_map: HashMap<OutputId, ExtractFieldOutput>,
    create_resource_map: HashMap<OutputId, CreateResourceOutput>,
}

impl OutputRepositoryImpl {
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

impl OutputRepository for OutputRepositoryImpl {
    fn get_native_functions_to_map(&self) -> Vec<FunctionsToMap> {
        self.native_function_map
            .iter()
            .flat_map(|(output_id, native_function)| {
                self.done_map
                    .get(&native_function.output_id)
                    .map(|done_output| {
                        FunctionsToMap::new(
                            *output_id,
                            native_function.native_function_id.clone(),
                            done_output.value.clone(),
                        )
                    })
            })
            .collect()
    }
    fn get_fields_to_extract(&self) -> Vec<FieldsToExtract> {
        self.extract_fields_map
            .iter()
            .flat_map(|(output_id, extract_fields)| {
                self.done_map
                    .get(&extract_fields.output_id)
                    .map(|done_output| {
                        FieldsToExtract::new(
                            *output_id,
                            extract_fields.field_name.clone(),
                            done_output.value.clone(),
                        )
                    })
            })
            .collect()
    }
    fn distribute_nothings(&mut self) -> bool {
        let native_functions_with_nothing = self
            .native_function_map
            .iter()
            .flat_map(|(output_id, native_function)| {
                self.nothing_map
                    .get(&native_function.output_id)
                    .map(|nothing| (*output_id, nothing.clone()))
            })
            .collect::<Vec<_>>();
        let extract_fields_with_nothing = self
            .extract_fields_map
            .iter()
            .flat_map(|(output_id, native_function)| {
                self.nothing_map
                    .get(&native_function.output_id)
                    .map(|nothing| {
                        (
                            *output_id,
                            nothing.clone(),
                            native_function.dependencies.clone(),
                        )
                    })
            })
            .collect::<Vec<_>>();

        let are_there_changes =
            !(native_functions_with_nothing.is_empty() && extract_fields_with_nothing.is_empty());

        for (output_id, nothing_output) in native_functions_with_nothing {
            self.native_function_map.remove(&output_id);
            self.nothing_map.insert(output_id, nothing_output);
        }
        for (output_id, nothing_output, dependencies) in extract_fields_with_nothing {
            let mut all_dependencies = BTreeSet::new();
            all_dependencies.extend(dependencies);
            all_dependencies.extend(nothing_output.dependencies.clone());

            self.extract_fields_map.remove(&output_id);
            self.nothing_map.insert(
                output_id,
                NothingOutput::new(all_dependencies.into_iter().collect()),
            );
        }
        are_there_changes
    }
}

#[cfg(test)]
mod tests {
    use rmpv::Value;
    use uuid::Uuid;

    use crate::model::{FieldName, NativeFunctionId};

    use super::*;

    mod get_native_functions_to_map {
        use super::*;

        #[test]
        fn should_return_native_functions_that_depend_on_done() {
            let mut output_repository = OutputRepositoryImpl::new();
            let done_output_id = OutputId::new(UUID_1);
            let native_function_output_id = OutputId::new(UUID_2);
            let native_function_id = NativeFunctionId::new("native_function_id".into());

            output_repository.done_map.insert(
                done_output_id,
                DoneOutput::new(Value::Boolean(true), vec!["dep1".into()]),
            );
            output_repository.native_function_map.insert(
                native_function_output_id,
                NativeFunctionOutput::new(done_output_id, native_function_id.clone()),
            );

            let mappable_native_functions = output_repository.get_native_functions_to_map();

            assert_eq!(
                mappable_native_functions,
                vec![FunctionsToMap::new(
                    native_function_output_id,
                    native_function_id,
                    Value::Boolean(true),
                )]
            );
        }
    }

    mod get_fields_to_extract {
        use super::*;

        #[test]
        fn should_return_extract_fields_that_depend_on_done() {
            let mut output_repository = OutputRepositoryImpl::new();
            let done_output_id = OutputId::new(UUID_1);
            let extract_field_output_id = OutputId::new(UUID_2);
            let field_name = FieldName::new("field_name".into());

            output_repository.done_map.insert(
                done_output_id,
                DoneOutput::new(Value::Boolean(true), vec!["dep1".into()]),
            );
            output_repository.extract_fields_map.insert(
                extract_field_output_id,
                ExtractFieldOutput::new(
                    done_output_id,
                    field_name.clone(),
                    vec!["extract1".into()],
                ),
            );

            let fields_to_extract = output_repository.get_fields_to_extract();

            assert_eq!(
                fields_to_extract,
                vec![FieldsToExtract::new(
                    extract_field_output_id,
                    field_name,
                    Value::Boolean(true),
                )]
            );
        }
    }

    mod distribute_nothings {
        use super::*;

        #[test]
        fn does_not_modify_done_outputs() {
            let mut output_repository = OutputRepositoryImpl::new();
            let native_function_output_id = OutputId::new(UUID_1);
            let native_function_id = NativeFunctionId::new("native_function_1_id".into());
            let done_output_id = OutputId::new(UUID_2);

            output_repository.done_map.insert(
                done_output_id,
                DoneOutput::new(Value::Boolean(true), vec!["done1".into()]),
            );

            output_repository.native_function_map.insert(
                native_function_output_id,
                NativeFunctionOutput::new(done_output_id, native_function_id.clone()),
            );

            let result = output_repository.distribute_nothings();

            assert!(!result);
            assert_eq!(
                output_repository.native_function_map,
                HashMap::from([(
                    native_function_output_id,
                    NativeFunctionOutput::new(done_output_id, native_function_id)
                ),])
            );
            assert_eq!(
                output_repository.done_map,
                HashMap::from([(
                    done_output_id,
                    DoneOutput::new(Value::Boolean(true), vec!["done1".into()])
                )])
            );
        }

        #[test]
        fn should_return_false_when_there_are_no_changes() {
            let mut output_repository = OutputRepositoryImpl::new();
            let result = output_repository.distribute_nothings();
            assert!(!result);
        }

        #[test]
        fn should_distribute_nothings_to_native_function() {
            let mut output_repository = OutputRepositoryImpl::new();
            let native_function_output_id = OutputId::new(UUID_1);
            let native_function_id = NativeFunctionId::new("native_function_id".into());
            let nothing_output_id = OutputId::new(UUID_2);

            output_repository.nothing_map.insert(
                nothing_output_id,
                NothingOutput::new(vec!["nothing1".into()]),
            );
            output_repository.native_function_map.insert(
                native_function_output_id,
                NativeFunctionOutput::new(nothing_output_id, native_function_id.clone()),
            );

            let result = output_repository.distribute_nothings();

            assert!(result);
            assert_eq!(output_repository.native_function_map, HashMap::new());
            assert_eq!(
                output_repository.nothing_map,
                HashMap::from([
                    (
                        nothing_output_id,
                        NothingOutput::new(vec!["nothing1".into()])
                    ),
                    (
                        native_function_output_id,
                        NothingOutput::new(vec!["nothing1".into()])
                    ),
                ])
            );
        }

        #[test]
        fn should_distribute_nothings_to_extract_fields_function() {
            let mut output_repository = OutputRepositoryImpl::new();
            let extract_field_output_id = OutputId::new(UUID_1);
            let field_name = FieldName::new("field_name".into());
            let nothing_output_id = OutputId::new(UUID_2);

            output_repository.nothing_map.insert(
                nothing_output_id,
                NothingOutput::new(vec!["nothing1".into()]),
            );
            output_repository.extract_fields_map.insert(
                extract_field_output_id,
                ExtractFieldOutput::new(nothing_output_id, field_name, vec!["extract1".into()]),
            );

            let result = output_repository.distribute_nothings();

            assert!(result);
            assert_eq!(output_repository.native_function_map, HashMap::new());
            assert_eq!(
                output_repository.nothing_map,
                HashMap::from([
                    (
                        nothing_output_id,
                        NothingOutput::new(vec!["nothing1".into()])
                    ),
                    (
                        extract_field_output_id,
                        NothingOutput::new(vec!["extract1".into(), "nothing1".into()])
                    ),
                ])
            );
        }
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
}
