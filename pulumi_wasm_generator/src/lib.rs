use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use anyhow::Result;

mod model;
mod output;
mod schema;

pub fn generate_rust_library(schema_json: &Path, result_path: &Path) -> Result<()> {
    let file = File::open(schema_json)?;
    let reader = BufReader::new(file);
    let package_json: schema::Package = serde_json::from_reader(reader)?;
    let package = schema::to_model(&package_json)?;

    fs::create_dir_all(result_path.join("wit").join("deps"))?;
    fs::create_dir_all(result_path.join("src"))?;

    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    wit_file.write_all(output::wit::generate_wit(&package)?.as_ref())?;

    let mut deps_wit_file =
        File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;
    deps_wit_file.write_all(output::wit::get_dependencies().as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::rust::cargo::generate_cargo(&package).as_bytes())?;

    let mut lib_file = File::create(result_path.join("src").join("lib.rs"))?;
    lib_file
        .write_all(output::rust::source_code_librs::generate_source_code(&package).as_bytes())?;

    let mut source_file = File::create(result_path.join("src").join("resource.rs"))?;
    source_file
        .write_all(output::rust::source_code_resource::generate_source_code(&package).as_bytes())?;

    let mut types_file = File::create(result_path.join("src").join("types.rs"))?;
    types_file
        .write_all(output::rust::source_code_types::generate_source_code(&package).as_ref())?;

    Ok(())
}

pub fn generate_wasm_provider(schema_json: &Path, result_path: &Path) -> Result<()> {
    let file = File::open(schema_json)?;
    let reader = BufReader::new(file);
    let package_json: schema::Package = serde_json::from_reader(reader)?;
    let package = schema::to_model(&package_json)?;

    fs::create_dir_all(result_path.join("wit").join("deps"))?;
    fs::create_dir_all(result_path.join("src"))?;

    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    wit_file.write_all(output::wit::generate_wit(&package)?.as_ref())?;

    let mut deps_wit_file =
        File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;
    deps_wit_file.write_all(output::wit::get_dependencies().as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::provider::cargo::generate_cargo(&package).as_bytes())?;

    let mut lib_file = File::create(result_path.join("src").join("lib.rs"))?;
    lib_file.write_all(output::provider::source_code::generate_source_code(&package).as_bytes())?;

    Ok(())
}
