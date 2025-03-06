use crate::model::{ElementId, Package};
use crate::output::functions::source_code_function_code::generate_single_function_source_code;
use crate::utils::reformat_code;
use anyhow::Context;

mod source_code_function_code;

pub(crate) fn generate_single_file(package: &Package, element_id: &ElementId) -> String {
    reformat_code(&generate_single_function_source_code(package, element_id))
        .context("Failed to reformat function source code")
        .unwrap()
}
