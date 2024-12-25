mod source_code_resource_code;

use crate::model::{ElementId, Package};
use crate::output::resources::source_code_resource_code::{
    generate_docs, generate_single_resource_source_code,
};
use crate::utils::reformat_code;
use anyhow::Context;

pub(crate) fn generate_single_file(package: &Package, element_id: &ElementId) -> String {
    reformat_code(&generate_single_resource_source_code(package, element_id))
        .context("Failed to reformat resource source code")
        .unwrap()
}

pub(crate) fn generate_single_file_docs(package: &Package, element_id: &ElementId) -> Vec<String> {
    generate_docs(package, element_id)
}
