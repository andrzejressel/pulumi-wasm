use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tempfile;

use pulumi_wasm_generator_lib::{
    extract_micro_package, generate_combined, generate_rust_library, generate_wasm_provider,
};
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

fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        Command::GenProvider {
            schema,
            output: destination,
            remove,
        } => {
            check_if_schema_exists(schema.as_ref())?;
            check_if_not_empty(destination.as_ref(), remove)?;
            generate_wasm_provider(schema.as_ref(), destination.as_ref())?;
        }
        Command::GenRust {
            schema,
            output: destination,
            remove,
        } => {
            check_if_schema_exists(schema.as_ref())?;
            check_if_not_empty(destination.as_ref(), remove)?;
            generate_rust_library(schema.as_ref(), destination.as_ref())?;
        }
    };

    Ok(())
}

fn check_if_schema_exists(schema: &Path) -> Result<()> {
    if !schema.exists() {
        Err(anyhow::anyhow!(
            "Schema file [{}] does not exist",
            schema.display()
        ))
    } else if !schema.is_file() {
        Err(anyhow::anyhow!(
            "Schema [{}] is not a file",
            schema.display()
        ))
    } else {
        Ok(())
    }
}

fn check_if_not_empty(output_directory: &Path, remove: Option<bool>) -> Result<()> {
    let remove = remove.unwrap_or(false);
    if output_directory.exists() && remove {
        fs::remove_dir_all(output_directory).context(format!(
            "Cannot remove directory [{}]",
            output_directory.display()
        ))?;
    }
    fs::create_dir_all(output_directory).context(format!(
        "Cannot create directory [{}]",
        output_directory.display()
    ))?;
    let is_empty = output_directory
        .read_dir()
        .context(format!(
            "Cannot read directory [{}]",
            output_directory.display()
        ))?
        .next()
        .is_none();
    if !is_empty {
        Err(anyhow::anyhow!(
            "Directory \"{}\" is not empty",
            output_directory.display()
        ))
    } else {
        Ok(())
    }
}
