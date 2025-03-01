use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GenerateRepoChangelog {
        new_version: String,
    }
}

fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        Command::GenerateRepoChangelog { new_version } => {
            let options = changelog_lib::Options {
                repository_path: Path::new("."),
                start_commit_id: "abacc7d01efb8cb7af5b19279b2b123a98b76a95",
                repository: "andrzejressel/pulumi-gestalt",
                changelog_dir: ".changelog",
            };
            let s = changelog_lib::generate_changelog_for_new_version(&options, &new_version).context("Failed to generate changelog")?;
            fs::write("CHANGELOG.md", &s).context("Failed to write changelog")?;
        }
    }

    Ok(())
}
