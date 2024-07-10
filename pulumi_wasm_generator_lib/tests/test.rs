use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;

use pulumi_wasm_generator_lib::{generate_rust_library, generate_wasm_provider};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
const REGENERATE_TESTS: bool = false;

#[test]
fn random() -> Result<()> {
    run_real_provider_test("random")
}

#[test]
fn command() -> Result<()> {
    run_real_provider_test("command")
}

#[test]
fn docker() -> Result<()> {
    run_real_provider_test("docker")
}

#[test]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative", "aws-native")
}

#[test]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types", "example")
}

// provider_name is `name` from yaml file
fn run_pulumi_generator_test(test_name: &str, provider_name: &str) -> Result<()> {
    let root_path = format!("tests/output/pulumi_tests/{test_name}");
    let root = Path::new(&root_path);
    let provider_output_path = root.join("provider");
    let output = Path::new(&provider_output_path);
    fs::create_dir_all(&root.join("lib"))?;

    let schema = find_schema_files(test_name);

    fs::copy(schema.clone(), root.join(schema.file_name().unwrap()))?;

    // schema.file_name().unwrap()

    generate_wasm_provider(schema.as_path(), output)?;
    generate_rust_library(schema.as_path(), &root.join("lib"))?;

    fs::copy(
        "tests/input/Cargo.toml",
        format!("tests/output/pulumi_tests/{test_name}/Cargo.toml"),
    )?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;

    Command::new("cargo")
        .args([
            "component",
            "build",
            "-p",
            format!("pulumi_wasm_{provider_name}_provider").as_str(),
        ])
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args([
            "build",
            "-p",
            format!("pulumi_wasm_{provider_name}").as_str(),
        ])
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

fn find_schema_files(name: &str) -> PathBuf {
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

fn run_real_provider_test(provider_name: &str) -> Result<()> {
    let root_path = format!("tests/output/pulumi_providers/{provider_name}");
    let root = Path::new(&root_path);
    let provider_output_path = root.join("provider");
    let output = Path::new(&provider_output_path);
    generate_wasm_provider(
        Path::new(&format!(
            "tests/schemas/pulumi-resource-{provider_name}.json"
        )),
        output,
    )?;
    generate_rust_library(
        Path::new(&format!(
            "tests/schemas/pulumi-resource-{provider_name}.json"
        )),
        &root.join("lib"),
    )?;

    fs::copy(
        "tests/input/Cargo.toml",
        format!("tests/output/pulumi_providers/{provider_name}/Cargo.toml"),
    )?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;

    Command::new("cargo")
        .args([
            "component",
            "build",
            "-p",
            format!("pulumi_wasm_{provider_name}_provider").as_str(),
        ])
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args([
            "build",
            "-p",
            format!("pulumi_wasm_{provider_name}").as_str(),
        ])
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}
