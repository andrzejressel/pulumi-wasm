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
#[cfg_attr(not(feature = "generator_array-of-enum-map"), ignore)]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map")
}

#[test]
#[cfg_attr(not(feature = "generator_azure-native-nested-types"), ignore)]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test("azure-native-nested-types")
}

#[test]
#[cfg_attr(not(feature = "generator_cyclic-types"), ignore)]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types")
}

#[test]
#[cfg_attr(not(feature = "generator_different-enum"), ignore)]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum")
}

#[test]
#[cfg_attr(not(feature = "generator_functions-secrets"), ignore)]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets")
}

#[test]
#[cfg_attr(not(feature = "generator_mini-awsnative"), ignore)]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative")
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module"), ignore)]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module")
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module-thirdparty"), ignore)]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty")
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs"), ignore)]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs")
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs-edgeorder"), ignore)]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder")
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-defaults"), ignore)]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults")
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-disable-defaults"), ignore)]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-disable-defaults")
}

#[test]
#[cfg_attr(not(feature = "generator_reserved_names"), ignore)]
fn reserved_names() -> Result<()> {
    run_pulumi_generator_test("reserved_names")
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inline"), ignore)]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline")
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inside-arrays"), ignore)]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays")
}

#[test]
#[cfg_attr(not(feature = "generator_workarounds"), ignore)]
fn workarounds() -> Result<()> {
    run_pulumi_generator_test("workarounds")
}

#[test]
#[cfg_attr(not(feature = "generator_cloudflare"), ignore)]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare")
}

#[test]
#[cfg_attr(not(feature = "generator_docker"), ignore)]
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker")
}

#[test]
#[cfg_attr(not(feature = "generator_gcp"), ignore)]
fn gcp() -> Result<()> {
    run_pulumi_generator_test("gcp")
}

#[test]
#[cfg_attr(not(feature = "generator_random"), ignore)]
fn random() -> Result<()> {
    run_pulumi_generator_test("random")
}
// DO NOT EDIT - END

// provider_name is `name` from yaml file
pub fn run_pulumi_generator_test(test_name: &str) -> Result<()> {
    let root_path = format!("tests/output/{test_name}");
    let root = Path::new(&root_path);

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

    Command::new("cargo")
        .args(["component", "build"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["test", "--doc"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["doc", "--no-deps"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("test_cases").join(format!("{name}.json")),
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
