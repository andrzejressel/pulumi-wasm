use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;

use pulumi_wasm_generator_lib::{generate_rust_library, generate_wasm_provider};
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn random() -> Result<()> {
    let provider_name = "random";
    run_test(provider_name)?;
    Ok(())
}

#[test]
fn command() -> Result<()> {
    let provider_name = "command";
    run_test(provider_name)?;
    Ok(())
}

#[test]
fn docker() -> Result<()> {
    let provider_name = "docker";
    run_test(provider_name)?;
    Ok(())
}

fn run_test(provider_name: &str) -> Result<()> {
    let root_path = format!("tests/output/{provider_name}_provider");
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
        format!("tests/output/{provider_name}_provider/Cargo.toml"),
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
