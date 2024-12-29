use anyhow::{Context, Result};
use assert_cmd::assert::OutputAssertExt;
use pulumi_wasm_generator::generate_combined;
use std::fs;
use std::fs::{File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
// DO NOT EDIT - START

#[test]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map")
}

#[test]
fn aws() -> Result<()> {
    run_pulumi_generator_test("aws")
}

#[test]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test("azure-native-nested-types")
}

#[test]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare")
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
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker")
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
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module")
}

#[test]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty")
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
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults")
}

#[test]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-disable-defaults")
}

#[test]
fn random() -> Result<()> {
    run_pulumi_generator_test("random")
}

#[test]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline")
}

#[test]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays")
}
// DO NOT EDIT - END

pub fn run_pulumi_generator_test(test_name: &str) -> Result<()> {
    let root_path = format!("tests/output/{test_name}");
    let root = Path::new(&root_path);

    eprintln!("Generating");
    let schema = find_schema_files(test_name);
    fs::create_dir_all(root)?;

    fs::copy(schema.clone(), root.join(schema.file_name().unwrap()))?;

    generate_combined(schema.as_path(), &root.join("src").join("generated"))?;

    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);

    let lib_rs = root.join("src/lib.rs");
    fs::copy("tests/input/Cargo.toml", root.join("Cargo.toml"))?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(&lib_rs, "include!(\"generated/main.rs\");")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;
    File::options()
        .write(true)
        .open(lib_rs)
        .context("Cannot open file")?
        .set_times(times)
        .context("Cannot set times")?;

    eprintln!("Compiling");
    Command::new("cargo")
        .args(["component", "build"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("../providers").join(format!("{name}.json")),
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
