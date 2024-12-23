use std::fs;
use std::process::Command;

#[derive(Debug)]
struct Provider<'a> {
    name: &'a str,
    version: &'a str,
}

fn main() {
    let providers = vec![
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
    let tests = vec![
        "array-of-enum-map",
        "azure-native-nested-types",
        "cyclic-types",
        "different-enum",
        "functions-secrets",
        "mini-awsnative",
        "output-funcs",
        "output-funcs-edgeorder",
        "unions-inline",
        "unions-inside-arrays",
        "plain-object-defaults",
        "plain-object-disable-defaults",
        "nested-module",
        "nested-module-thirdparty",
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

    update_cargo_toml(&providers);
    update_justfile(&providers);
    update_tests(&tests);
}

fn update_tests(tests: &[&str]) {
    update_github_actions_build(tests);
    update_test_rs(tests);
}

fn update_github_actions_build(tests: &[&str]) {
    let content = fs::read_to_string(".github/workflows/deploy.yml")
        .expect("Failed to read .github/workflows/deploy.yml");

    let mut replacement = String::new();
    replacement.push_str("            ./\n");
    for test in tests {
        replacement.push_str(&format!(
            "            pulumi_wasm_generator_lib/tests/output/{}/\n",
            test
        ));
    }
    let start_marker = " # DO NOT EDIT - START 1";
    let end_marker = "# DO NOT EDIT - END 1";
    let content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write(".github/workflows/deploy.yml", content)
        .expect("Failed to write to .github/workflows/deploy.yml");
}

fn update_test_rs(tests: &[&str]) {
    let content = fs::read_to_string("pulumi_wasm_generator_lib/tests/test.rs")
        .expect("Failed to read pulumi_wasm_generator_lib/tests/test.rs");

    let mut replacement = String::new();
    for test_directory in tests {
        let method_name = test_directory.replace("-", "_");

        let code = format!(
            r#"
#[test]
fn {method_name}() -> Result<()> {{
    run_pulumi_generator_test("{test_directory}")
}}
"#
        );

        replacement.push_str(&code);
    }
    let start_marker = "// DO NOT EDIT - START";
    let end_marker = "// DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write("pulumi_wasm_generator_lib/tests/test.rs", new_content)
        .expect("Failed to write to pulumi_wasm_generator_lib/tests/test.rs");
}

fn update_cargo_toml(providers: &[Provider]) {
    let content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    let mut replacement = String::new();
    for provider in providers {
        replacement.push_str(&format!(
            "    \"providers/pulumi_wasm_provider_{}\",\n",
            provider.name
        ));
        replacement.push_str(&format!(
            "    \"providers/pulumi_wasm_provider_{}_rust\",\n",
            provider.name
        ));
    }
    let start_marker = "    # DO NOT EDIT - START";
    let end_marker = "    # DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write("Cargo.toml", new_content).expect("Failed to write to Cargo.toml");
}

fn update_justfile(providers: &[Provider]) {
    let content = fs::read_to_string("justfile").expect("Failed to read justfile");
    let content = replace_regenerate_providers(providers, &content);
    let content = replace_build_wasm_components(providers, &content);
    let content = replace_publish_wasm_components(providers, &content);
    let content = replace_generate_rust_docs(providers, &content);

    fs::write("justfile", content).expect("Failed to write to justfile");
}

fn replace_regenerate_providers(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    for provider in providers {
        replacement.push_str(&format!("    cargo run -p pulumi_wasm_generator -- gen-provider --remove true --schema providers/{}.json --output providers/pulumi_wasm_provider_{}\n", provider.name, provider.name));
        replacement.push_str(&format!("    cargo run -p pulumi_wasm_generator -- gen-rust     --remove true --schema providers/{}.json --output providers/pulumi_wasm_provider_{}_rust\n", provider.name, provider.name));
    }

    let start_marker =
        "# DO NOT EDIT - REGENERATE-PROVIDERS - START\nregenerate-providers-generated:";
    let end_marker = "# DO NOT EDIT - REGENERATE-PROVIDERS - END";
    replace_between_markers(content, start_marker, end_marker, &replacement)
}

fn replace_build_wasm_components(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    replacement.push_str("build-wasm-providers:\n");
    replacement.push_str("    cargo component build \\\n");
    for provider in providers {
        replacement.push_str(&format!(
            "      -p pulumi_wasm_{}_provider \\\n",
            provider.name
        ));
    }
    replacement.push_str("      --timings\n");
    replacement.push('\n');
    replacement.push_str("build-wasm-providers-release:\n");
    replacement.push_str("    cargo component build \\\n");
    for provider in providers {
        replacement.push_str(&format!(
            "      -p pulumi_wasm_{}_provider \\\n",
            provider.name
        ));
    }
    replacement.push_str("      --timings --release\n");

    let start_marker = "# DO NOT EDIT - BUILD-WASM-COMPONENTS - START";
    let end_marker = "# DO NOT EDIT - BUILD-WASM-COMPONENTS - END";
    replace_between_markers(content, start_marker, end_marker, &replacement)
}

fn replace_publish_wasm_components(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    replacement.push_str("publish-providers:\n");
    for provider in providers {
        replacement.push_str(&format!(
            "    cargo publish -p pulumi_wasm_{}\n",
            provider.name
        ));
    }
    let start_marker = "# DO NOT EDIT - PUBLISH-PROVIDERS - START";
    let end_marker = "# DO NOT EDIT - PUBLISH-PROVIDERS - END";
    replace_between_markers(content, start_marker, end_marker, &replacement)
}

fn replace_generate_rust_docs(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    replacement.push_str("rust-docs:\n");
    replacement.push_str("    cargo test --doc");
    for provider in providers {
        // Docker docs are untested, because Pulumi can't get their shit together:
        // https://github.com/pulumi/pulumi-docker/issues/1278
        if provider.name != "docker" {
            replacement.push_str(&format!(" -p pulumi_wasm_{}", provider.name));
        }
    }
    replacement.push('\n');
    replacement.push_str("    cargo doc --no-deps -p pulumi_wasm_rust");
    for provider in providers {
        replacement.push_str(&format!(" -p pulumi_wasm_{}", provider.name));
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
