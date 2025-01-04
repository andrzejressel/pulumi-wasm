use anyhow::{Context, Result};
use assert_cmd::assert::OutputAssertExt;
use pulumi_wasm_generator::generate_combined;
use rinja::Template;
use std::fs;
use std::fs::{File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
// DO NOT EDIT - START

#[test]
#[cfg_attr(not(feature = "generator_array-of-enum-map"), ignore)]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map", "array-of-enum-map", None)
}

#[test]
#[cfg_attr(not(feature = "generator_azure-native-nested-types"), ignore)]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test(
        "azure-native-nested-types",
        "azure-native-nested-types",
        None,
    )
}

#[test]
#[cfg_attr(not(feature = "generator_cloudflare"), ignore)]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare", "cloudflare", None)
}

#[test]
#[cfg_attr(not(feature = "generator_cyclic-types"), ignore)]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types", "cyclic-types", None)
}

#[test]
#[cfg_attr(not(feature = "generator_different-enum"), ignore)]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum", "different-enum", None)
}

#[test]
#[cfg_attr(not(feature = "generator_docker"), ignore)]
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker", "docker", None)
}

#[test]
#[cfg_attr(not(feature = "generator_functions-secrets"), ignore)]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets", "functions-secrets", None)
}

#[test]
#[cfg_attr(not(feature = "generator_mini-awsnative"), ignore)]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative", "mini-awsnative", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module"), ignore)]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module", "nested-module", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module-thirdparty"), ignore)]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty", "nested-module-thirdparty", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs"), ignore)]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs", "output-funcs", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs-edgeorder"), ignore)]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder", "output-funcs-edgeorder", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-defaults"), ignore)]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults", "plain-object-defaults", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-disable-defaults"), ignore)]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test(
        "plain-object-disable-defaults",
        "plain-object-disable-defaults",
        None,
    )
}

#[test]
#[cfg_attr(not(feature = "generator_random"), ignore)]
fn random() -> Result<()> {
    run_pulumi_generator_test("random", "random", None)
}

#[test]
#[cfg_attr(not(feature = "generator_reserved_names"), ignore)]
fn reserved_names() -> Result<()> {
    run_pulumi_generator_test("reserved_names", "reserved_names", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inline"), ignore)]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline", "unions-inline", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inside-arrays"), ignore)]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays", "unions-inside-arrays", None)
}

#[test]
#[cfg_attr(not(feature = "generator_workarounds"), ignore)]
fn workarounds() -> Result<()> {
    run_pulumi_generator_test("workarounds", "workarounds", None)
}

#[test]
#[cfg_attr(not(feature = "generator_azure-0"), ignore)]
fn azure_0() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-0",
        Some(&[
            "aadb2c",
            "advisor",
            "analysisservices",
            "apimanagement",
            "appconfiguration",
            "appinsights",
            "appplatform",
            "appservice",
            "arc",
            "arckubernetes",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-1"), ignore)]
fn azure_1() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-1",
        Some(&[
            "arcmachine",
            "armmsi",
            "attestation",
            "authorization",
            "automanage",
            "automation",
            "avs",
            "backup",
            "batch",
            "billing",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-2"), ignore)]
fn azure_2() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-2",
        Some(&[
            "blueprint",
            "bot",
            "cdn",
            "chaosstudio",
            "cognitive",
            "communication",
            "compute",
            "confidentialledger",
            "connections",
            "consumption",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-3"), ignore)]
fn azure_3() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-3",
        Some(&[
            "containerapp",
            "containerservice",
            "core",
            "cosmosdb",
            "costmanagement",
            "customip",
            "dashboard",
            "databasemigration",
            "databoxedge",
            "databricks",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-4"), ignore)]
fn azure_4() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-4",
        Some(&[
            "datadog",
            "datafactory",
            "dataprotection",
            "datashare",
            "desktopvirtualization",
            "devcenter",
            "devtest",
            "digitaltwins",
            "dns",
            "domainservices",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-5"), ignore)]
fn azure_5() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-5",
        Some(&[
            "dynatrace",
            "elasticcloud",
            "elasticsan",
            "eventgrid",
            "eventhub",
            "expressroute",
            "extendedlocation",
            "fluidrelay",
            "frontdoor",
            "graph",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-6"), ignore)]
fn azure_6() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-6",
        Some(&[
            "hdinsight",
            "healthcare",
            "hpc",
            "hsm",
            "iot",
            "iotcentral",
            "keyvault",
            "kusto",
            "lb",
            "lighthouse",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-7"), ignore)]
fn azure_7() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-7",
        Some(&[
            "loadtest",
            "loganalytics",
            "logicapps",
            "machinelearning",
            "maintenance",
            "managedapplication",
            "managedlustre",
            "management",
            "managementgroups",
            "managementresource",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-8"), ignore)]
fn azure_8() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-8",
        Some(&[
            "maps",
            "marketplace",
            "mixedreality",
            "mobile",
            "monitoring",
            "msi",
            "mssql",
            "mysql",
            "netapp",
            "network",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-9"), ignore)]
fn azure_9() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-9",
        Some(&[
            "networkfunction",
            "newrelic",
            "nginx",
            "notificationhub",
            "operationalinsights",
            "oracle",
            "orbital",
            "paloalto",
            "pim",
            "policy",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-10"), ignore)]
fn azure_10() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-10",
        Some(&[
            "portal",
            "postgresql",
            "powerbi",
            "privatedns",
            "privatelink",
            "proximity",
            "purview",
            "recoveryservices",
            "redhatopenshift",
            "redis",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-11"), ignore)]
fn azure_11() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-11",
        Some(&[
            "relay",
            "role",
            "search",
            "securitycenter",
            "sentinel",
            "servicebus",
            "servicefabric",
            "signalr",
            "siterecovery",
            "stack",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-12"), ignore)]
fn azure_12() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-12",
        Some(&[
            "storage",
            "streamanalytics",
            "synapse",
            "systemcenter",
            "trafficmanager",
            "trustedsigning",
            "videoindexer",
            "voice",
            "waf",
            "webpubsub",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-13"), ignore)]
fn azure_13() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-13", Some(&["workloadssap"]))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-0"), ignore)]
fn filtering_0() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-0", Some(&["ns1"]))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-1"), ignore)]
fn filtering_1() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-1", Some(&["ns2"]))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-2"), ignore)]
fn filtering_2() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-2", Some(&["ns1", "ns2"]))
}
// DO NOT EDIT - END

// provider_name is `name` from yaml file
pub fn run_pulumi_generator_test(
    schema_name: &str,
    directory_name: &str,
    modules: Option<&[&str]>,
) -> Result<()> {
    let root_path = format!("tests/output/{directory_name}");
    let root = Path::new(&root_path);

    let schema = find_schema_files(schema_name);
    fs::create_dir_all(root)?;

    create_symlink(&schema, &root.join(schema.file_name().unwrap()))?;

    generate_combined(
        schema.as_path(),
        &root.join("src").join("generated"),
        modules,
    )?;

    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);

    let cargo_toml_content = CargoToml {
        name: directory_name,
    }
    .render()?;
    let lib_rs = root.join("src/lib.rs");
    fs::write(root.join("Cargo.toml"), cargo_toml_content)?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(&lib_rs, "include!(\"generated/main.rs\");")?;
    fs::copy("../rust-toolchain.toml", root.join("rust-toolchain.toml"))?;

    if let Some(env) = std::env::var_os("DO_NOT_COMPILE") {
        if env == "true" {
            if !root.join("Cargo.lock").exists() {
                Command::new("cargo")
                    .args(["generate-lockfile"])
                    .env_remove("CARGO_LLVM_COV")
                    .env_remove("RUSTFLAGS")
                    .current_dir(root)
                    .assert()
                    .success();
            }
            return Ok(());
        }
    }

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
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["test", "--doc"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["doc", "--no-deps"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("tests/test_cases").join(format!("{name}.json")),
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

fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_file(dst)?;
    }
    use pathdiff::diff_paths;
    let relative_path = diff_paths(src, dst.parent().unwrap()).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&relative_path, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&relative_path, dst)?;
    Ok(())
}

#[derive(Template)]
#[template(path = "test_Cargo.toml.jinja")]
struct CargoToml<'a> {
    name: &'a str,
}
