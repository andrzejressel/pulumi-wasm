use itertools::Itertools;
use std::fs;
use std::process::Command;

#[derive(Debug)]
struct Provider<'a> {
    name: &'a str,
    version: &'a str,
}

fn main() {
    let mut providers = vec![
        Provider {
            name: "aws",
            version: "6.66.2",
        },
        Provider {
            name: "docker",
            version: "4.5.3",
        },
        Provider {
            name: "random",
            version: "4.15.0",
        },
        Provider {
            name: "cloudflare",
            version: "5.43.1",
        },
    ];
    providers.sort_by(|a, b| a.name.cmp(b.name));
    let tests = vec![
        "array-of-enum-map",
        "azure-native-nested-types",
        "cyclic-types",
        "different-enum",
        "functions-secrets",
        "mini-awsnative",
        "nested-module",
        "nested-module-thirdparty",
        "output-funcs",
        "output-funcs-edgeorder",
        "plain-object-defaults",
        "plain-object-disable-defaults",
        "reserved_names",
        "unions-inline",
        "unions-inside-arrays",
    ];
    for provider in &providers {
        println!("{:?}", provider);
        let schema_output = Command::new("pulumi")
            .arg("package")
            .arg("get-schema")
            .arg(format!("{}@{}", provider.name, provider.version))
            .output()
            .expect("Failed to execute pulumi command");

        let schema =
            String::from_utf8(schema_output.stdout).expect("Invalid UTF-8 in pulumi output");

        fs::write(format!("providers/{}.json", provider.name), schema)
            .expect("Failed to write schema to file");
    }

    update_justfile(&providers);
    update_tests(&tests, &providers);
    update_generator_cargo_toml(&tests, &providers);
}

fn update_tests(tests: &[&str], providers: &[Provider]) {
    update_github_actions_build(tests, providers);
    update_test_rs(tests, providers);
}

fn update_github_actions_build(tests: &[&str], providers: &[Provider]) {
    let content = fs::read_to_string(".github/workflows/build.yml")
        .expect("Failed to read .github/workflows/build.yml");

    let mut replacement = String::new();
    replacement.push_str("        provider: [");
    replacement
        .push_str(&itertools::chain!(providers.iter().map(|p| &p.name), tests.iter()).join(", "));
    replacement.push_str("]\n");
    let start_marker = "# DO NOT EDIT - PROVIDER START";
    let end_marker = "# DO NOT EDIT - PROVIDER END";
    let content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write(".github/workflows/build.yml", content)
        .expect("Failed to write to .github/workflows/build.yml");
}

fn update_test_rs(tests: &[&str], providers: &[Provider]) {
    let content = fs::read_to_string("pulumi_wasm_generator/tests/test.rs")
        .expect("Failed to read pulumi_wasm_generator/tests/test.rs");

    let mut replacement = String::new();
    for test_directory in tests {
        let method_name = test_directory.replace("-", "_");

        let code = format!(
            r#"
#[test]
#[cfg_attr(not(feature = "generator_{test_directory}"), ignore)]
fn {method_name}() -> Result<()> {{
    run_pulumi_generator_test("{test_directory}")
}}
"#
        );

        replacement.push_str(&code);
    }

    for provider in providers {
        let method_name = provider.name.replace("-", "_");
        let provider_name = provider.name;

        let code = format!(
            r#"
#[test]
#[cfg_attr(not(feature = "generator_{provider_name}"), ignore)]
fn {method_name}() -> Result<()> {{
    run_pulumi_generator_test("{provider_name}")
}}
"#
        );
        replacement.push_str(&code);
    }

    let start_marker = "// DO NOT EDIT - START";
    let end_marker = "// DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write("pulumi_wasm_generator/tests/test.rs", new_content)
        .expect("Failed to write to pulumi_wasm_generator/tests/test.rs");
}

fn update_generator_cargo_toml(tests: &[&str], providers: &[Provider]) {
    let content =
        fs::read_to_string("pulumi_wasm_generator/Cargo.toml").expect("Failed to read Cargo.toml");
    let mut replacement = String::new();
    for provider in providers {
        replacement.push_str(&format!("generator_{} = []\n", provider.name))
    }
    for test in tests {
        replacement.push_str(&format!("generator_{} = []\n", test))
    }
    let start_marker = "# DO NOT EDIT - START";
    let end_marker = "# DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);
    fs::write("pulumi_wasm_generator/Cargo.toml", new_content)
        .expect("Failed to write to pulumi_wasm_generator/Cargo.toml");
}

fn update_justfile(providers: &[Provider]) {
    let content = fs::read_to_string("justfile").expect("Failed to read justfile");
    let content = replace_generate_rust_docs(providers, &content);

    fs::write("justfile", content).expect("Failed to write to justfile");
}

fn replace_generate_rust_docs(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    replacement.push_str("rust-docs:\n");
    replacement.push_str("    cargo doc --no-deps -p pulumi_wasm_rust -p pulumi_wasm_build");
    for provider in providers {
        replacement.push_str(&format!(" -p pulumi_wasm_providers_{}", provider.name));
    }
    replacement.push('\n');
    let start_marker = "# DO NOT EDIT - GENERATE-RUST-DOCS - START";
    let end_marker = "# DO NOT EDIT - GENERATE-RUST-DOCS - END";
    replace_between_markers(content, start_marker, end_marker, &replacement)
}

fn replace_between_markers(
    source: &str,
    start_marker: &str,
    end_marker: &str,
    replacement: &str,
) -> String {
    let start_index = source
        .find(start_marker)
        .expect("Start marker not found in source");
    let end_index = source
        .find(end_marker)
        .expect("End marker not found in source");

    let mut new_content = String::new();
    new_content.push_str(&source[..start_index + start_marker.len()]);
    new_content.push('\n');
    new_content.push_str(replacement);
    new_content.push_str(&source[end_index..]);

    new_content
}
