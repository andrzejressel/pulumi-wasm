use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let options = changelog_lib::Options {
        repository_path: Path::new("."),
        start_commit_id: "abacc7d01efb8cb7af5b19279b2b123a98b76a95",
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: ".changelog",
    };
    let s = changelog_lib::generate_changelog(&options).context("Failed to generate changelog")?;
    fs::write("CHANGELOG.md", &s).context("Failed to write changelog")?;

    Ok(())
}
