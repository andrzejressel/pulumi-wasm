use std::fs;
use std::path::Path;
use std::process::Command;
use assert_cmd::assert::OutputAssertExt;

#[test]
fn random() -> anyhow::Result<()> {
    pulumi_wasm_generator::generate_files(
        Path::new("tests/schemas/pulumi-resource-random.json"),
        Path::new("tests/output/random")
    )?;

    fs::create_dir_all("tests/output/random/src")?;
    fs::create_dir_all("tests/output/random/deps/pulumi-wasm")?;
    fs::write("tests/output/random/src/lib.rs", "")?;

    Command::new("cargo")
        .args(["component", "build"])
        .current_dir("tests/output/random")
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