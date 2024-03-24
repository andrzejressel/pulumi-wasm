use std::fs;
use std::path::Path;
use std::process::Command;
use assert_cmd::assert::OutputAssertExt;
use anyhow::Result;

#[test]
fn random() -> Result<()> {
    let provider_name = "random";
    run_test(provider_name.to_string())?;
    Ok(())
}

#[test]
fn command() -> Result<()> {
    let provider_name = "command";
    run_test(provider_name.to_string())?;
    Ok(())
}

// #[test]
// fn aws() -> Result<()> {
//     let provider_name = "aws";
//     run_test(provider_name.to_string())?;
//     Ok(())
// }

fn run_test(provider_name: String) -> Result<()> {
    pulumi_wasm_generator::generate_files(
        Path::new(&format!("tests/schemas/pulumi-resource-{provider_name}.json")),
        Path::new(&format!("tests/output/{provider_name}"))
    )?;

    fs::create_dir_all(format!("tests/output/{provider_name}/src"))?;
    fs::create_dir_all(format!("tests/output/{provider_name}/deps/pulumi-wasm"))?;
    fs::write(format!("tests/output/{provider_name}/src/lib.rs"), "")?;

    Command::new("cargo")
        .args(["component", "build"])
        .current_dir(format!("tests/output/{provider_name}"))
        .assert()
        .success();
    Ok(())
}

// #[test]
// fn command() -> anyhow::Result<()> {
//     pulumi_wasm_generator::generate_files(
//         std::path::Path::new("tests/schemas/pulumi-resource-command.json"),
//         std::path:: Path::new("tests/result.json")
//     )?;
//     Ok(())
// }