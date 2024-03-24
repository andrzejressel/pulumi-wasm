mod schema;
mod model;
mod output;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use serde::Deserialize;

pub fn generate_files(path: &Path, result_path: &Path) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package_json: schema::Package = serde_json::from_reader(reader)?;
    let package = schema::to_model(&package_json);

    fs::create_dir_all(result_path.join("wit").join("deps"))?;

    // let mut wit_file = File::create(result_path.join(format!("{}.wit", package.name)))?;
    let mut wit_file = File::create(result_path.join("wit").join("world.wit"))?;
    let wit = output::wit::generate_wit(&package)?;

    let mut deps_wit_file = File::create(result_path.join("wit").join("deps").join("pulumi-wasm.wit"))?;

    wit_file.write_all(wit.as_bytes())?;
    deps_wit_file.write_all(output::wit::get_dependencies().as_ref())?;

    let mut cargo_file = File::create(result_path.join("Cargo.toml"))?;
    cargo_file.write_all(output::rust::cargo::generate_cargo(&package).as_bytes())?;

    Ok(())
}