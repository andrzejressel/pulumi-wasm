use anyhow::Context;
use anyhow::Result;
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
    GenerateRepoChangelog { new_version: String },
    GenerateForDocs {},
    DryRun {},
}

fn main() -> Result<()> {
    let args = App::parse();
    let options = changelog_lib::Options {
        repository_path: Path::new("."),
        start_commit_id: "abacc7d01efb8cb7af5b19279b2b123a98b76a95",
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: ".changelog",
    };

    match args.command {
        Command::GenerateRepoChangelog { new_version } => {
            let s = changelog_lib::generate_changelog_for_new_version(&options, &new_version)
                .context("Failed to generate changelog")?;
            fs::write("CHANGELOG.md", &s).context("Failed to write changelog")?;
            let s = changelog_lib::generate_changelog_for_github_changelog(&options, &new_version)
                .context("Failed to generate changelog")?;
            fs::write("target/github_changelog.md", &s).context("Failed to write GitHub changelog")?;
        }
        Command::GenerateForDocs {} => {
            let s = changelog_lib::generate_mkdocs_changelog(&options)
                .context("Failed to generate changelog")?;
            fs::write("docs/CHANGELOG.md", &s).context("Failed to write changelog")?;
        }
        Command::DryRun {} => {
            changelog_lib::generate_changelog(&options)
                .context("Failed to generate repo changelog")?;
            changelog_lib::generate_mkdocs_changelog(&options)
                .context("Failed to generate mkdocs changelog")?;
            changelog_lib::generate_changelog_for_github_changelog(&options, "9999.0.0")
                .context("Failed to generate github changelog")?;
            changelog_lib::generate_changelog_for_new_version(&options, "9999.0.0")
                .context("Failed to generate new version changelog")?;
        }
    }

    Ok(())
}
