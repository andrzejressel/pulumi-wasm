use anyhow::{Context, Result};
use pulumi_wasm_generator_lib::{extract_micro_package, generate_combined};
use std::path::Path;
use std::{env, fs};

pub fn generate(provider_name: &str, provider_version: &str) -> Result<()> {
    let schema_output = std::process::Command::new("pulumi")
        .arg("package")
        .arg("get-schema")
        .arg(format!("{}@{}", provider_name, provider_version))
        .output()
        .context("Failed to execute pulumi command")?;

    let schema = String::from_utf8(schema_output.stdout).expect("Invalid UTF-8 in pulumi output");

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;

    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
    let file = temp_dir.path().join("schema.json");
    fs::write(&file, &schema).context("Failed to write schema")?;

    generate_combined(file.as_path(), &location).context("Failed to generate glue files")?;
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

pub fn generate_from_schema(schema_file: &Path) -> Result<()> {
    let package = extract_micro_package(schema_file).context("Failed to deserialize package")?;
    let provider_name = package.name;

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;
    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    generate_combined(schema_file, &location).context("Failed to generate glue files")?;
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", schema_file.display());
    Ok(())
}
