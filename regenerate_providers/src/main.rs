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

    fs::write("justfile", content).expect("Failed to write to justfile");
}

fn replace_regenerate_providers(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    for provider in providers {
        replacement.push_str(&format!("    cargo run -p pulumi_wasm_generator -- gen-provider --remove true --schema providers/{}.json --output providers/pulumi_wasm_provider_{}\n", provider.name, provider.name));
        replacement.push_str(&format!("    cargo run -p pulumi_wasm_generator -- gen-rust     --remove true --schema providers/{}.json --output providers/pulumi_wasm_provider_{}_rust\n", provider.name, provider.name));
    }

    let start_marker = "# DO NOT EDIT - REGENERATE-PROVIDERS - START\nregenerate-providers:";
    let end_marker = "# DO NOT EDIT - REGENERATE-PROVIDERS - END";
    replace_between_markers(content, start_marker, end_marker, &replacement)
}

fn replace_build_wasm_components(providers: &[Provider], content: &str) -> String {
    let mut replacement = String::new();
    for provider in providers {
        replacement.push_str(&format!(
            "      -p pulumi_wasm_{}_provider \\\n",
            provider.name
        ));
    }
    replacement.push_str("      --timings\n");
    let start_marker =
        "    # DO NOT EDIT - BUILD-WASM-COMPONENTS - START\n    cargo component build \\";
    let end_marker = "    # DO NOT EDIT - BUILD-WASM-COMPONENTS - END";
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
