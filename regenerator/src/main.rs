mod generate_proto;
mod provider;

use itertools::Itertools;
use std::fs;
use std::process::Command;

#[derive(Debug)]
struct Provider<'a> {
    name: &'a str,
    version: &'a str,
}

#[derive(Debug)]
struct FilteredTest<'a> {
    name: &'a str,
    filters: Vec<Vec<String>>,
}

fn main() {
    let azure_modules = provider::find_modules("azure").unwrap();
    let gcp_modules = provider::find_modules("gcp").unwrap();
    let aws_modules = provider::find_modules("aws").unwrap();

    let mut grouped_gcp = gcp_modules
        .into_iter()
        .filter(|s| s != &"compute")
        .chunks(10)
        .into_iter()
        .map(|a| a.collect_vec())
        .collect::<Vec<_>>();
    grouped_gcp.push(vec!["compute".to_string()]);
    let mut filtered_tests = vec![
        FilteredTest {
            name: "filtering",
            filters: vec![
                vec!["ns1".to_string()],
                vec!["ns2".to_string()],
                vec!["ns1".to_string(), "ns2".to_string()],
            ],
        },
        FilteredTest {
            name: "azure",
            filters: azure_modules
                .to_vec()
                .chunks(10)
                .map(|a| a.to_vec())
                .collect(),
        },
        FilteredTest {
            name: "gcp",
            filters: grouped_gcp,
        },
        FilteredTest {
            name: "aws",
            filters: aws_modules
                .to_vec()
                .chunks(10)
                .map(|a| a.to_vec())
                .collect(),
        },
    ];
    let mut providers = vec![
        Provider {
            name: "azure",
            version: "6.14.0",
        },
        Provider {
            name: "gcp",
            version: "8.12.1",
        },
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
            version: "4.15.1",
        },
        Provider {
            name: "cloudflare",
            version: "5.43.1",
        },
    ];
    filtered_tests.sort_by(|a, b| a.name.cmp(b.name));
    providers.sort_by(|a, b| a.name.cmp(b.name));
    let mut tests = vec![
        "array-of-enum-map",
        "azure-native-nested-types",
        "cloudflare",
        "cyclic-types",
        "docker",
        "different-enum",
        "functions-secrets",
        "mini-awsnative",
        "nested-module",
        "nested-module-thirdparty",
        "output-funcs",
        "output-funcs-edgeorder",
        "plain-object-defaults",
        "plain-object-disable-defaults",
        "random",
        "reserved_names",
        "unions-inline",
        "unions-inside-arrays",
        "workarounds",
    ];
    tests.sort();
    for provider in &providers {
        println!("{:?}", provider);
        let schema_output = Command::new("pulumi")
            .arg("package")
            .arg("get-schema")
            .arg(format!("{}@{}", provider.name, provider.version))
            .env("PULUMI_AWS_MINIMAL_SCHEMA", "true") // https://github.com/pulumi/pulumi-aws/issues/2565
            .output()
            .expect("Failed to execute pulumi command");

        let schema =
            String::from_utf8(schema_output.stdout).expect("Invalid UTF-8 in pulumi output");

        fs::write(format!("providers/{}.json", provider.name), schema)
            .expect("Failed to write schema to file");
    }

    update_tests(&tests, &filtered_tests);
    update_generator_cargo_toml(&tests, &filtered_tests);
    generate_proto::regenerate_proto().expect("Failed to regenerate proto");
}

fn update_tests(tests: &[&str], filtered_tests: &[FilteredTest]) {
    update_github_actions_build(tests, filtered_tests);
    update_test_rs(tests, filtered_tests);
}

fn update_github_actions_build(tests: &[&str], filtered_tests: &[FilteredTest]) {
    let content = fs::read_to_string(".github/workflows/build.yml")
        .expect("Failed to read .github/workflows/build.yml");

    let mut replacement = String::new();
    replacement.push_str("        provider: [");
    let mut test_names = tests.iter().map(|test| test.to_string()).collect_vec();
    for provider in filtered_tests {
        for (index, _) in provider.filters.iter().enumerate() {
            test_names.push(format!("{}-{}", provider.name, index).to_string());
        }
    }
    replacement.push_str(&test_names.join(", "));
    replacement.push_str("]\n");
    let start_marker = "# DO NOT EDIT - PROVIDER START";
    let end_marker = "# DO NOT EDIT - PROVIDER END";
    let content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write(".github/workflows/build.yml", content)
        .expect("Failed to write to .github/workflows/build.yml");
}

fn update_test_rs(tests: &[&str], filtered_tests: &[FilteredTest]) {
    let content = fs::read_to_string("crates/rust_generator/tests/test.rs")
        .expect("Failed to read crates/rust_generator/tests/test.rs");

    let mut replacement = String::new();
    for test_directory in tests {
        let method_name = test_directory.replace("-", "_");

        let code = format!(
            r#"
#[test]
#[cfg_attr(not(feature = "generator_{test_directory}"), ignore)]
fn {method_name}() -> Result<()> {{
    run_pulumi_generator_test("{test_directory}", "{test_directory}", None)
}}
"#
        );

        replacement.push_str(&code);
    }

    for filtered_test in filtered_tests {
        for (index, filter) in filtered_test.filters.iter().enumerate() {
            let provider_name = filtered_test.name;
            let feature_name = format!("generator_{}-{}", filtered_test.name, index);
            let directory_name = format!("{}-{}", filtered_test.name, index);
            let method_name = directory_name.replace("-", "_");
            let filter_name = filter.iter().map(|s| format!("\"{s}\"")).join(",");
            let code = format!(
                r#"
#[test]
#[cfg_attr(not(feature = "{feature_name}"), ignore)]
fn {method_name}() -> Result<()> {{
    run_pulumi_generator_test("{provider_name}", "{directory_name}", Some(&[{filter_name}]))
}}
"#
            );
            replacement.push_str(&code);
        }
    }

    let start_marker = "// DO NOT EDIT - START";
    let end_marker = "// DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);

    fs::write("crates/rust_generator/tests/test.rs", new_content)
        .expect("Failed to write to crates/rust_generator/tests/test.rs");
}

fn update_generator_cargo_toml(tests: &[&str], filtered_tests: &[FilteredTest]) {
    let content =
        fs::read_to_string("crates/rust_generator/Cargo.toml").expect("Failed to read Cargo.toml");
    let mut replacement = String::new();
    for test in tests {
        replacement.push_str(&format!("generator_{} = []\n", test))
    }
    for filtered_test in filtered_tests {
        for (index, _) in filtered_test.filters.iter().enumerate() {
            replacement.push_str(&format!(
                "generator_{}-{} = []\n",
                filtered_test.name, index
            ))
        }
    }

    let start_marker = "# DO NOT EDIT - START";
    let end_marker = "# DO NOT EDIT - END";
    let new_content = replace_between_markers(&content, start_marker, end_marker, &replacement);
    fs::write("crates/rust_generator/Cargo.toml", new_content)
        .expect("Failed to write to crates/rust_generator/Cargo.toml");
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
