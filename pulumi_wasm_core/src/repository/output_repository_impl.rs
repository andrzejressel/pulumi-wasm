use std::collections::HashMap;

use crate::model::{
    DoneOutput, ExtractFieldOutput, FieldsToExtract, FunctionsToMap,
    NativeFunctionOutput, NothingOutput, OutputId,
};
use crate::repository::output_repository::OutputRepository;

#[derive(Default)]
pub(crate) struct OutputRepositoryImpl {
    // Ready values
    nothing_map: HashMap<OutputId, NothingOutput>,
    done_map: HashMap<OutputId, DoneOutput>,

    // Values waiting to be computed
    native_function_map: HashMap<OutputId, NativeFunctionOutput>,
    extract_fields_map: HashMap<OutputId, ExtractFieldOutput>,
}

impl OutputRepositoryImpl {}

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
    fn distribute_nothings(&mut self) {
        let native_functions_with_nothing = self
            .native_function_map
            .iter()
            .flat_map(|(output_id, native_function)| {
                self.nothing_map.get(&native_function.output_id)
                    .map(|nothing| (*output_id, nothing.clone()))
            })
            .collect::<Vec<_>>();

        for (output_id, nothing_output) in native_functions_with_nothing {
            self.native_function_map.remove(&output_id);
            self.nothing_map.insert(output_id, nothing_output);
        }

    }
}

#[cfg(test)]
mod tests {
    use rmpv::Value;
    use uuid::Uuid;

    use crate::model::{FieldName, NativeFunctionId};

    use super::*;

    #[test]
    fn should_return_native_functions_that_depend_on_done() {
        let mut output_repository = OutputRepositoryImpl::default();
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

    #[test]
    fn should_return_extract_fields_that_depend_on_done() {
        let mut output_repository = OutputRepositoryImpl::default();
        let done_output_id = OutputId::new(UUID_1);
        let extract_field_output_id = OutputId::new(UUID_2);
        let field_name = FieldName::new("field_name".into());

        output_repository.done_map.insert(
            done_output_id,
            DoneOutput::new(Value::Boolean(true), vec!["dep1".into()]),
        );
        output_repository.extract_fields_map.insert(
            extract_field_output_id,
            ExtractFieldOutput::new(done_output_id, field_name.clone(), vec!["extract1".into()]),
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

    #[test]
    fn should_spread_nothings_to_native_functions() {
        let mut output_repository = OutputRepositoryImpl::default();
        let native_function_1_output_id = OutputId::new(UUID_1);
        let native_function_1_id = NativeFunctionId::new("native_function_1_id".into());
        let native_function_2_output_id = OutputId::new(UUID_2);
        let native_function_2_id = NativeFunctionId::new("native_function_2_id".into());
        let done_output_id = OutputId::new(UUID_3);
        let nothing_output_id = OutputId::new(UUID_4);

        output_repository.done_map.insert(
            done_output_id,
            DoneOutput::new(Value::Boolean(true), vec!["done1".into()]),
        );
        output_repository
            .nothing_map
            .insert(nothing_output_id, NothingOutput::new(vec!["nothing1".into()]));

        output_repository.native_function_map.insert(
            native_function_1_output_id,
            NativeFunctionOutput::new(done_output_id, native_function_1_id.clone()),
        );
        output_repository.native_function_map.insert(
            native_function_2_output_id,
            NativeFunctionOutput::new(nothing_output_id, native_function_2_id.clone()),
        );

        output_repository.distribute_nothings();

        assert_eq!(
            output_repository.native_function_map,
            HashMap::from([
                (native_function_1_output_id, NativeFunctionOutput::new(done_output_id, native_function_1_id)),
            ])
        );
        assert_eq!(
            output_repository.nothing_map,
            HashMap::from([
                (nothing_output_id, NothingOutput::new(vec!["nothing1".into()])),
                (native_function_2_output_id, NothingOutput::new(vec!["nothing1".into()])),
            ])
        );
        assert_eq!(
            output_repository.done_map,
            HashMap::from([
                (done_output_id, DoneOutput::new(Value::Boolean(true), vec!["done1".into()]))
            ])
        );
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
}
