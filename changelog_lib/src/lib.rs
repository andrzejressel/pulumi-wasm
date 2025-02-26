use anyhow::{Context, Result};
use bon::Builder;
use gix::reference::Category;
use std::error::Error;
use std::path::Path;

mod model;

#[derive(Builder)]
struct Options<'a> {
    repository_path: &'a Path,
    start_commit_id: &'a str,
}

pub fn generate_changelog(new_version: &str, repository: &Path) -> Result<()> {
    let repo = gix::open(repository)
        .with_context(|| format!("Failed to open git repository in {}", repository.display()))?;

    // Get the HEAD reference (current branch)
    let mut head_ref = repo.head().context("Failed to get HEAD reference")?;
    // let head_commit = head_ref.peel_to_commit()?;

    let mut commit_id_to_tag = std::collections::HashMap::new();

    for reference in repo
        .references()
        .context("Failed to get references")?
        .all()
        .context("Failed to get all references")?
    {
        let mut reference = match reference {
            Ok(r) => r,
            Err(e) => anyhow::bail!("Failed to get reference: {}", e),
        };

        if reference.name().category() == Some(Category::Tag) {
            println!("tag: {}", reference.name().shorten());
            commit_id_to_tag.insert(reference.peel_to_commit()?.id().to_string(), reference.name().shorten().to_string());
        }
    }

    let commit_id_to_tag = commit_id_to_tag;

    let head_commit = head_ref
        .peel_to_commit_in_place()
        .context("Failed to peel HEAD reference to commit")?;

    let rev_walk = repo.rev_walk(vec![head_commit.id]);

    for commit in rev_walk.all().context("Failed to iterate commits")? {
        let commit = commit.context("Failed to get commit")?;
        println!("commit: {}", commit.id);
    }

    println!("Mapping: {:?}", commit_id_to_tag);

    Ok(())
}

fn prepare_changelog_directory() {}
