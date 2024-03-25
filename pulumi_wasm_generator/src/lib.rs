mod schema;
mod model;
mod output;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn generate_files(path: &Path, result_path: &Path) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package_json: schema::Package = serde_json::from_reader(reader)?;
    let package = schema::to_model(&package_json);

    fs::create_dir_all(result_path.join("wit").join("deps"))?;
    fs::create_dir_all(result_path.join("src"))?;

    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    wit_file.write_all(output::wit::generate_wit(&package)?.as_ref())?;

    let mut deps_wit_file = File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;
    deps_wit_file.write_all(output::wit::get_dependencies().as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::provider::cargo::generate_cargo(&package).as_bytes())?;

    let mut lib_file = File::create(result_path.join("src").join("lib.rs"))?;
    lib_file.write_all(output::provider::source_code::generate_source_code(&package).as_bytes())?;

    Ok(())
}