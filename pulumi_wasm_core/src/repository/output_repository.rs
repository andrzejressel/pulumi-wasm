#[cfg(test)]
use mockall::automock;

use crate::model::{DoneOutput, FieldsToExtract, FunctionsToMap, OutputId};

#[cfg_attr(test, automock)]
pub(crate) trait OutputRepository {
    fn add_done_output(&mut self, output_id: OutputId, output: DoneOutput);
    fn get_native_functions_to_map(&self) -> Vec<FunctionsToMap>;
    fn get_fields_to_extract(&self) -> Vec<FieldsToExtract>;
    // Returns true if there were changes
    fn distribute_nothings(&mut self) -> bool;
}
