use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use std::fs;
use std::path::Path;

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
            pulumi_wasm_generator::generate_wasm_provider(schema.as_ref(), destination.as_ref())?;
        }
        Command::GenRust {
            schema,
            output: destination,
            remove,
        } => {
            check_if_schema_exists(schema.as_ref())?;
            check_if_not_empty(destination.as_ref(), remove)?;
            pulumi_wasm_generator::generate_rust_library(schema.as_ref(), destination.as_ref())?;
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
