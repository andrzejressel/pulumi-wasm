use anyhow::{Context, Result};
use assert_cmd::assert::OutputAssertExt;
use pulumi_wasm_generator_lib::{generate_rust_library, generate_wasm_provider};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

// DO NOT EDIT - START

#[test]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map")
}

#[test]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types")
}

#[test]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum")
}

#[test]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets")
}

#[test]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative")
}

#[test]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs")
}

#[test]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder")
}

#[test]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline")
}

#[test]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays")
}

#[test]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults")
}

#[test]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-disable-defaults")
}

#[test]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module")
}

#[test]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty")
}
// DO NOT EDIT - END

// provider_name is `name` from yaml file
pub fn run_pulumi_generator_test(test_name: &str) -> Result<()> {
    let root_path = format!("tests/output/{test_name}");
    let root = Path::new(&root_path);
    let provider_output_path = root.join("provider");
    let output = Path::new(&provider_output_path);
    fs::create_dir_all(root.join("lib"))?;

    let schema = find_schema_files(test_name);
    let micropackage = extract_micro_package(&schema)?;

    fs::copy(schema.clone(), root.join(schema.file_name().unwrap()))?;

    generate_wasm_provider(schema.as_path(), output)?;
    generate_rust_library(schema.as_path(), &root.join("lib"))?;

    fs::copy("tests/input/Cargo.toml", root.join("Cargo.toml"))?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;

    Command::new("cargo")
        .args([
            "component",
            "build",
            "-p",
            format!("pulumi_wasm_{}_provider", micropackage.name).as_str(),
        ])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args([
            "build",
            "-p",
            format!("pulumi_wasm_{}", micropackage.name).as_str(),
        ])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("../pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.yaml"),
        Path::new("../pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.json"),
        Path::new("../pulumi/tests/testdata/codegen").join(format!("{name}.yaml")),
        Path::new("../pulumi/tests/testdata/codegen").join(format!("{name}.json")),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.yaml"),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.json"),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata").join(format!("{name}.yaml")),
        Path::new("../pulumi-java/pkg/codegen/testing/test/testdata").join(format!("{name}.json")),
    ];

    for path in possible_paths {
        if path.exists() {
            return path;
        }
    }

    panic!("No schema file found for provider: {name}");
}

fn extract_micro_package(schema_json: &Path) -> anyhow::Result<MicroPackage> {
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
struct MicroPackage {
    name: String,
}
