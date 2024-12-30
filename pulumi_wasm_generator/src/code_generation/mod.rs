use crate::code_generation::rust_generation::generate_code;
use crate::code_generation::yaml::model::yaml_to_model;
use crate::code_generation::yaml::yaml_model::YamlFile;
use anyhow::Context;
pub(crate) mod rust_generation;
mod tests;
pub(crate) mod yaml;

pub fn generate_code_from_string(
    yaml: String,
    package: &crate::model::Package,
) -> anyhow::Result<String> {
    let yaml_file =
        YamlFile::from_yaml(yaml.as_str()).context(format!("Failed to parse YAML: {}", yaml))?;
    let example = yaml_to_model(yaml_file, package.name.clone(), package)
        .with_context(|| format!("Failed to convert YAML {} to model", yaml))?;
    generate_code(example)
}
