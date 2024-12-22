use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use crate::schema::Package;
use crate::schema_types::PulumiSchema;
use anyhow::{Context, Result};

mod code_generation;
mod description;
mod model;
mod output;
mod schema;
mod schema_types;
mod utils;

pub fn generate_rust_library(schema_json: &Path, result_path: &Path) -> Result<()> {
    let schema_package: schema::Package = extract_schema_from_file(schema_json)?;
    extract_schema_2_from_file(schema_json)?;
    let package = schema::to_model(&schema_package)?;

    fs::create_dir_all(result_path.join("wit").join("deps"))?;
    fs::create_dir_all(result_path.join("src"))?;
    fs::create_dir_all(result_path.join("src").join("resource"))?;
    fs::create_dir_all(result_path.join("src").join("types"))?;
    fs::create_dir_all(result_path.join("src").join("function"))?;

    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    wit_file.write_all(output::wit::generate_wit(&package)?.as_ref())?;

    let mut deps_wit_file =
        File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;
    deps_wit_file.write_all(output::wit::get_dependencies()?.as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::rust::cargo::generate_cargo(&package).as_bytes())?;

    let mut lib_file = File::create(result_path.join("src").join("lib.rs"))?;
    lib_file
        .write_all(output::rust::source_code_librs::generate_source_code(&package).as_bytes())?;

    let mut source_file = File::create(result_path.join("src").join("resource").join("mod.rs"))?;
    source_file.write_all(
        output::rust::source_code_resource_mod::generate_source_code(&package).as_bytes(),
    )?;

    let mut source_file = File::create(result_path.join("src").join("function").join("mod.rs"))?;
    source_file.write_all(
        output::rust::source_code_function_mod::generate_source_code(&package).as_bytes(),
    )?;

    output::rust::source_code_types_code::generate_source_code(&package)
        .iter()
        .for_each(|(path, content)| {
            let mut lib_file =
                File::create(result_path.join("src").join("types").join(path)).unwrap();
            lib_file.write_all(content.as_bytes()).unwrap();
        });

    // let mut types_file = File::create(result_path.join("src").join("types.rs"))?;
    // types_file
    //     .write_all(output::rust::source_code_types::generate_source_code(&package).as_ref())?;

    File::create(result_path.join("src").join("types").join("mod.rs"))?
        .write_all(output::rust::source_code_types_mod::generate_source_code(&package).as_ref())?;

    output::rust::source_code_resource_code::generate_source_code(&package)
        .iter()
        .for_each(|(path, content)| {
            let mut lib_file =
                File::create(result_path.join("src").join("resource").join(path)).unwrap();
            lib_file.write_all(content.as_bytes()).unwrap();
        });

    output::rust::source_code_function_code::generate_source_code(&package)
        .iter()
        .for_each(|(path, content)| {
            let mut lib_file =
                File::create(result_path.join("src").join("function").join(path)).unwrap();
            lib_file.write_all(content.as_bytes()).unwrap();
        });

    Ok(())
}

pub fn generate_wasm_provider(schema_json: &Path, result_path: &Path) -> Result<()> {
    let schema_package: schema::Package = extract_schema_from_file(schema_json)?;
    extract_schema_2_from_file(schema_json)?;
    let package = schema::to_model(&schema_package)?;

    fs::create_dir_all(result_path.join("wit").join("deps"))?;
    fs::create_dir_all(result_path.join("src").join("resource"))?;
    fs::create_dir_all(result_path.join("src").join("function"))?;

    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    wit_file.write_all(output::wit::generate_wit(&package)?.as_ref())?;

    let mut deps_wit_file =
        File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;
    deps_wit_file.write_all(output::wit::get_dependencies()?.as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::provider::cargo::generate_cargo(&package).as_bytes())?;

    let mut lib_file = File::create(result_path.join("src").join("lib.rs"))?;
    lib_file.write_all(
        output::provider::source_code_librs::generate_source_code(&package).as_bytes(),
    )?;

    let mut source_file = File::create(result_path.join("src").join("resource").join("mod.rs"))?;
    source_file.write_all(
        output::provider::source_code_resource_mod::generate_source_code(&package).as_bytes(),
    )?;

    output::provider::source_code_resource_code::generate_source_code(&package)
        .iter()
        .for_each(|(path, content)| {
            let mut lib_file =
                File::create(result_path.join("src").join("resource").join(path)).unwrap();
            lib_file.write_all(content.as_bytes()).unwrap();
        });

    let mut source_file = File::create(result_path.join("src").join("function").join("mod.rs"))?;
    source_file.write_all(
        output::provider::source_code_function_mod::generate_source_code(&package).as_bytes(),
    )?;

    output::provider::source_code_function_code::generate_source_code(&package)
        .iter()
        .for_each(|(path, content)| {
            let mut lib_file =
                File::create(result_path.join("src").join("function").join(path)).unwrap();
            lib_file.write_all(content.as_bytes()).unwrap();
        });

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

fn extract_schema_2_from_file(schema_json: &Path) -> anyhow::Result<PulumiSchema> {
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
