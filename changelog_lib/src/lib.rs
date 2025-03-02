use crate::encoders::{GithubFlavorEncoder, MkdocsEncoder};
use crate::model::{ChangelogEntry, ChangelogType, GitHistory, TagName};
use anyhow::{bail, Context, Result};
use bon::Builder;
use encoders::Encoder;
use gix::bstr::ByteSlice;
use gix::reference::Category;
use model::Version;
use std::fs;
use std::path::Path;

mod encoders;
mod model;

#[derive(Builder)]
pub struct Options<'a> {
    pub repository_path: &'a Path,
    pub start_commit_id: &'a str,
    pub repository: &'a str,
    pub changelog_dir: &'a str,
}

pub fn generate_mkdocs_changelog(options: &Options) -> Result<String> {
    let history = generate_history(options, None).context("Failed to generate history")?;
    let s = generate_changelog_content(history, options, MkdocsEncoder {})
        .context("Failed to generate changelog content")?;
    Ok(s)
}

pub fn generate_changelog(options: &Options) -> Result<String> {
    let history = generate_history(options, None).context("Failed to generate history")?;
    let s = generate_changelog_content(history, options, GithubFlavorEncoder {})
        .context("Failed to generate changelog content")?;
    Ok(s)
}

pub fn generate_changelog_for_new_version(options: &Options, new_version: &str) -> Result<String> {
    let history = generate_history(options, Some(new_version.to_string()))
        .context("Failed to generate history")?;
    let s = generate_changelog_content(history, options, GithubFlavorEncoder {})
        .context("Failed to generate changelog content")?;
    Ok(s)
}

pub fn generate_changelog_for_github_changelog(options: &Options, version: &str) -> Result<String> {
    let history = generate_history(options, None).context("Failed to generate history")?;
    let version = history
        .versions
        .into_iter()
        .find(|v| v.tag_name.is_real_tag_with_version(version))
        .context(format!("Failed to find version [{}]", version))?;
    let new_history = GitHistory {
        versions: vec![version.clone()],
    };
    let s = generate_changelog_content(new_history, options, GithubFlavorEncoder {})
        .context("Failed to generate changelog content")?;
    Ok(s)
}

fn generate_changelog_content(
    history: GitHistory,
    options: &Options,
    encoder: impl Encoder,
) -> Result<String> {
    let mut s = String::new();

    let changelog_dir = options.repository_path.join(options.changelog_dir);

    for (index, version) in history.versions.iter().enumerate() {
        let previous_version = history.versions.get(index + 1).map(|v| v.tag_name.clone());

        match (&version.tag_name, previous_version) {
            (new_version, None) => {
                s.push_str(&format!("## {}\n", new_version.get_version_name()));
            }
            (new_version, Some(prev_version)) => {
                s.push_str(&format!(
                    "## [{}](https://github.com/{}/compare/{}...{})\n",
                    new_version.get_version_name(),
                    options.repository,
                    prev_version.get_tag(),
                    new_version.get_tag()
                ));
            }
        }

        let version_dir = changelog_dir.join(version.tag_name.get_changelog_yaml_directory());

        if version_dir.exists() {
            let mut added = vec![];
            let mut changed = vec![];
            let mut deprecated = vec![];
            let mut removed = vec![];
            let mut fixed = vec![];
            let mut security = vec![];

            let files = fs::read_dir(&version_dir)
                .with_context(|| format!("Failed to read directory {}", version_dir.display()))?;

            for file in files {
                let file = file.with_context(|| {
                    format!(
                        "Failed to read file from directory {}",
                        version_dir.display()
                    )
                })?;

                let path = file.path();
                if path.extension().and_then(|f| f.to_str()) != Some("yaml") {
                    continue;
                }

                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read file {}", file.path().display()))?;
                let entry: ChangelogEntry = serde_yaml::from_str(&content)
                    .with_context(|| format!("Failed to parse file {}", file.path().display()))?;

                match entry.r#type {
                    ChangelogType::Added => {
                        added.push(entry);
                    }
                    ChangelogType::Changed => {
                        changed.push(entry);
                    }
                    ChangelogType::Deprecated => {
                        deprecated.push(entry);
                    }
                    ChangelogType::Removed => {
                        removed.push(entry);
                    }
                    ChangelogType::Fixed => {
                        fixed.push(entry);
                    }
                    ChangelogType::Security => {
                        security.push(entry);
                    }
                }
            }

            if !added.is_empty() {
                s.push_str("### Added\n");
                for entry in added {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }

            if !changed.is_empty() {
                s.push_str("### Changed\n");
                for entry in changed {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }

            if !deprecated.is_empty() {
                s.push_str("### Deprecated\n");
                for entry in deprecated {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }

            if !removed.is_empty() {
                s.push_str("### Removed\n");
                for entry in removed {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }

            if !fixed.is_empty() {
                s.push_str("### Fixed\n");
                for entry in fixed {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }

            if !security.is_empty() {
                s.push_str("### Security\n");
                for entry in security {
                    s.push_str(&format!("- {}\n", entry.title));
                }
                s.push('\n');
            }
        }

        if !version.renovate_bot_commits.is_empty() {
            s.push_str(&encoder.encode_collapsible_block_start("🤖 Dependency updates"));

            for commit in &version.renovate_bot_commits {
                let line = format!(
                    "- {} [{}](https://github.com/{}/commit/{})",
                    commit.title,
                    commit.id.chars().take(7).collect::<String>(),
                    options.repository,
                    commit.id
                );

                s.push_str(&encoder.encode_collapsible_block_element(&line));
            }

            s.push_str(&encoder.encode_collapsible_block_end());
        }

        s.push_str(&encoder.encode_collapsible_block_start("Commits"));

        for commit in &version.commits {
            let line = format!(
                "- {} [{}](https://github.com/{}/commit/{})",
                commit.title,
                commit.id.chars().take(7).collect::<String>(),
                options.repository,
                commit.id
            );
            s.push_str(&encoder.encode_collapsible_block_element(&line));
        }

        s.push_str(&encoder.encode_collapsible_block_end());
    }
    Ok(s)
}

fn generate_history(options: &Options, new_version_name: Option<String>) -> Result<GitHistory> {
    let repo = gix::open(options.repository_path).with_context(|| {
        format!(
            "Failed to open git repository in {}",
            options.repository_path.display()
        )
    })?;

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
            Err(e) => bail!("Failed to get reference: {}", e),
        };

        if reference.name().category() == Some(Category::Tag) {
            println!("tag: {}", reference.name().shorten());
            commit_id_to_tag.insert(
                reference.peel_to_commit()?.id().to_string(),
                reference.name().shorten().to_string(),
            );
        }
    }

    let commit_id_to_tag = commit_id_to_tag;

    let head_commit = head_ref
        .peel_to_commit_in_place()
        .context("Failed to peel HEAD reference to commit")?;

    let rev_walk = repo.rev_walk(vec![head_commit.id]);

    let mut history = GitHistory { versions: vec![] };

    let mut version = Version {
        tag_name: match new_version_name {
            None => TagName::not_yet_released(),
            Some(v) => TagName::not_yet_released_with_version(v),
        },
        renovate_bot_commits: vec![],
        commits: vec![],
    };

    for commit in rev_walk.all().context("Failed to iterate commits")? {
        let commit = commit.context("Failed to get commit")?;

        let id = commit.id().to_string();

        if id == options.start_commit_id {
            break;
        }

        if let Some(tag) = commit_id_to_tag.get(&id) {
            // We don't want an empty `unreleased` section
            if !(matches!(version.tag_name, TagName::NotYetReleased) && version.commits.is_empty())
            {
                history.versions.push(version);
            }
            version = Version {
                tag_name: TagName::real_tag(tag.clone()),
                renovate_bot_commits: vec![],
                commits: vec![],
            };
        }

        let object = commit.object().context("Failed to get commit object")?;

        let message = object
            .message()
            .context("Failed to get commit message")?
            .summary()
            .to_str()
            .context("Failed to convert commit message to string")?
            .trim()
            .to_string();

        let author_email = object
            .author()
            .context("Failed to get commit author")?
            .email
            .to_string();

        let commit_model = model::Commit {
            id: commit.id().to_string(),
            title: message.clone(),
        };

        if author_email == "29139614+renovate[bot]@users.noreply.github.com" {
            version.renovate_bot_commits.push(commit_model.clone());
        }

        if !message.starts_with("[no-changelog]") {
            version.commits.push(commit_model);
        }

        println!("commit: {}", commit.id);
    }

    history.versions.push(version);

    println!("Mapping: {:?}", commit_id_to_tag);

    println!("History: {:?}", history);

    Ok(history)
}
