use crate::schema::Package;
use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod code_generation;
mod description;
mod model;
mod output;
mod schema;
mod utils;

pub fn generate_combined(schema_json: &Path, result_path: &Path) -> Result<()> {
    let schema_package: schema::Package = extract_schema_from_file(schema_json)?;
    let package = schema::to_model(&schema_package)?;

    fs::create_dir_all(result_path)?;

    output::generate_combined_code(&package, result_path);

    Ok(())
}

fn extract_schema_from_file(schema_json: &Path) -> anyhow::Result<Package> {
    let file = File::open(schema_json)
        .with_context(|| format!("Error opening schema file [{:?}]", schema_json))?;
    let reader = BufReader::new(file);
    match schema_json.extension().and_then(|s| s.to_str()) {
        None => Err(anyhow::anyhow!(
            "No extensions for schema file: {}.",
            schema_json.display()
        )),
        Some("yml" | "yaml") => serde_yaml::from_reader(reader).map_err(anyhow::Error::new),
        Some("json") => serde_json::from_reader(reader).map_err(anyhow::Error::new),
        Some(ext) => Err(anyhow::anyhow!(
            "Unsupported schema file extension: {}.",
            ext
        )),
    }
}

pub fn extract_micro_package(schema_json: &Path) -> anyhow::Result<MicroPackage> {
    let file = File::open(schema_json)
        .with_context(|| format!("Error opening schema file [{:?}]", schema_json))?;
    let reader = BufReader::new(file);
    match schema_json.extension().and_then(|s| s.to_str()) {
        None => Err(anyhow::anyhow!(
            "No extensions for schema file: {}.",
            schema_json.display()
        )),
        Some("yml" | "yaml") => serde_yaml::from_reader(reader).map_err(anyhow::Error::new),
        Some("json") => serde_json::from_reader(reader).map_err(anyhow::Error::new),
        Some(ext) => Err(anyhow::anyhow!(
            "Unsupported schema file extension: {}.",
            ext
        )),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MicroPackage {
    pub name: String,
}
