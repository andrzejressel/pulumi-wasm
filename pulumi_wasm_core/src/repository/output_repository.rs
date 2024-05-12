use crate::model::{FieldsToExtract, FunctionsToMap};

pub(crate) trait OutputRepository {
    fn get_native_functions_to_map(&self) -> Vec<FunctionsToMap>;
    fn get_fields_to_extract(&self) -> Vec<FieldsToExtract>;
    // Returns true if there were changes
    fn distribute_nothings(&mut self) -> bool;
}
