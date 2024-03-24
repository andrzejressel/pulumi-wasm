use std::fs;
use std::path::Path;
use std::process::Command;
use assert_cmd::assert::OutputAssertExt;
use anyhow::Result;

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

fn run_test(provider_name: &str) -> Result<()> {
    let output_path = format!("tests/output/{provider_name}_provider");
    let output = Path::new(&output_path);
    pulumi_wasm_generator::generate_files(
        Path::new(&format!("tests/schemas/pulumi-resource-{provider_name}.json")),
        &output
    )?;



    // fs::create_dir_all(output.join("src"))?;
    // fs::write(output.join("src/lib.rs"), "")?;

    Command::new("cargo")
        .args(["component", "build"])
        .current_dir(&output)
        .assert()
        .success();
    Ok(())
}
