use clap::{Parser, Subcommand};

use pulumi_wasm_generator_lib::{extract_micro_package, generate_combined};
use std::path::Path;
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GenProvider {
        #[arg(short, long)]
        schema: String,

        #[arg(short, long)]
        output: String,

        #[arg(short, long)]
        remove: Option<bool>,
    },
    GenRust {
        #[arg(short, long)]
        schema: String,

        #[arg(short, long)]
        output: String,

        #[arg(short, long)]
        remove: Option<bool>,
    },
}

pub fn generate(provider_name: &str, provider_version: &str) {
    let schema_output = std::process::Command::new("pulumi")
        .arg("package")
        .arg("get-schema")
        .arg(format!("{}@{}", provider_name, provider_version))
        .output()
        .expect("Failed to execute pulumi command");

    let schema = String::from_utf8(schema_output.stdout).expect("Invalid UTF-8 in pulumi output");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = out_dir.to_str().unwrap();
    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    let temp_dir = tempfile::tempdir().unwrap();
    let file = temp_dir.path().join("schema.json");
    fs::write(&file, &schema).unwrap();

    generate_combined(file.as_path(), &location).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}

pub fn generate_from_schema(schema_file: &Path) {
    let package = extract_micro_package(schema_file).unwrap();
    let provider_name = package.name;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = out_dir.to_str().unwrap();
    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    generate_combined(schema_file, &location).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
