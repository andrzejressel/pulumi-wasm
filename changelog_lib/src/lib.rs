use crate::encoders::{GithubFlavorEncoder, MkdocsEncoder};
use crate::model::{ChangelogEntry, ChangelogType, Commit, GitHistory, TagName};
use anyhow::{bail, format_err, Context, Result};
use bon::Builder;
use encoders::Encoder;
use gix::bstr::ByteSlice;
use gix::reference::Category;
use model::Version;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

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

            let mut entries = fs::read_dir(&version_dir)
                .with_context(|| format!("Failed to read directory {}", version_dir.display()))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to obtain entries paths")?;

            entries.sort();

            for path in entries {
                if path.extension().and_then(|f| f.to_str()) != Some("yaml") {
                    continue;
                }

                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read file {}", path.display()))?;
                let entry: ChangelogEntry = serde_yaml::from_str(&content)
                    .with_context(|| format!("Failed to parse file {}", path.display()))?;

                match entry.r#type {
                    ChangelogType::Added => {
                        added.push(ChangelogEntryWithPath { entry, path });
                    }
                    ChangelogType::Changed => {
                        changed.push(ChangelogEntryWithPath { entry, path });
                    }
                    ChangelogType::Deprecated => {
                        deprecated.push(ChangelogEntryWithPath { entry, path });
                    }
                    ChangelogType::Removed => {
                        removed.push(ChangelogEntryWithPath { entry, path });
                    }
                    ChangelogType::Fixed => {
                        fixed.push(ChangelogEntryWithPath { entry, path });
                    }
                    ChangelogType::Security => {
                        security.push(ChangelogEntryWithPath { entry, path });
                    }
                }
            }

            if !added.is_empty() {
                s.push_str("### Added\n");
                print_changelog_entries(options, &mut s, &added)?;
                s.push('\n');
            }

            if !changed.is_empty() {
                s.push_str("### Changed\n");
                print_changelog_entries(options, &mut s, &changed)?;
                s.push('\n');
            }

            if !deprecated.is_empty() {
                s.push_str("### Deprecated\n");
                print_changelog_entries(options, &mut s, &deprecated)?;
                s.push('\n');
            }

            if !removed.is_empty() {
                s.push_str("### Removed\n");
                print_changelog_entries(options, &mut s, &removed)?;
                s.push('\n');
            }

            if !fixed.is_empty() {
                s.push_str("### Fixed\n");
                print_changelog_entries(options, &mut s, &fixed)?;
                s.push('\n');
            }

            if !security.is_empty() {
                s.push_str("### Security\n");
                print_changelog_entries(options, &mut s, &security)?;
                s.push('\n');
            }
        }

        if !version.dependency_update_commits.is_empty() {
            s.push_str(&encoder.encode_collapsible_block_start("🤖 Dependency Update Commits"));

            for commit in &version.dependency_update_commits {
                let line = generate_commit_line(options, commit);
                s.push_str(&encoder.encode_collapsible_block_element(&line));
            }

            s.push_str(&encoder.encode_collapsible_block_end());
        }

        s.push_str(&encoder.encode_collapsible_block_start("📝 Other Commits"));

        for commit in &version.commits {
            let line = generate_commit_line(options, commit);
            s.push_str(&encoder.encode_collapsible_block_element(&line));
        }

        s.push_str(&encoder.encode_collapsible_block_end());
    }
    Ok(s)
}

fn print_changelog_entries(
    options: &Options,
    s: &mut String,
    entries: &Vec<ChangelogEntryWithPath>,
) -> Result<()> {
    for ChangelogEntryWithPath { entry, path } in entries {
        s.push_str(
            generate_commit_message(entry, path, options)
                .with_context(|| {
                    format!("Failed to generate changelog entry [{}]", path.display())
                })?
                .as_str(),
        );
    }
    Ok(())
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
        dependency_update_commits: vec![],
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
                dependency_update_commits: vec![],
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

        let commit_model = Commit {
            id: commit.id().to_string(),
            title: message.clone(),
        };

        if author_email == "29139614+renovate[bot]@users.noreply.github.com" {
            version.dependency_update_commits.push(commit_model);
        } else if author_email == "49699333+dependabot[bot]@users.noreply.github.com" {
            version.dependency_update_commits.push(commit_model);
        } else if !message.starts_with("[no-changelog]") {
            version.commits.push(commit_model);
        }
    }

    history.versions.push(version);
    Ok(history)
}

static PR_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(#(\d+)\)$").unwrap());

fn generate_commit_line(options: &Options, commit: &Commit) -> String {
    let commit_message = &commit.title;
    let commit_message = PR_ID_REGEX.replace_all(
        commit_message,
        format!("([#$1](https://github.com/{}/pull/$1))", options.repository),
    );

    format!(
        "- {} [{}](https://github.com/{}/commit/{})",
        commit_message,
        commit.id.chars().take(7).collect::<String>(),
        options.repository,
        commit.id
    )
}

fn generate_commit_message(
    entry: &ChangelogEntry,
    path: &Path,
    options: &Options,
) -> Result<String> {
    let mut title = PR_ID_REGEX
        .replace_all(
            &entry.title,
            format!("[#$1](https://github.com/{}/pull/$1)", options.repository),
        )
        .to_string();

    let (commit_sha, commit_message) = get_file_creation_commit(path, options)?;

    if let Some(captures) = PR_ID_REGEX.captures(&commit_message) {
        if let Some(number) = captures.get(1) {
            title.push_str(
                format!(
                    " ([#{}](https://github.com/{}/pull/{}))",
                    number.as_str(),
                    options.repository,
                    number.as_str()
                )
                .as_str(),
            );
        }
    }

    for pr_id in &entry.additional_pull_requests {
        title.push_str(
            format!(
                " ([#{}](https://github.com/{}/pull/{}))",
                pr_id, options.repository, pr_id
            )
            .as_str(),
        );
    }

    let short_sha = commit_sha.chars().take(7).collect::<String>();
    title.push_str(&format!(
        " [{}](https://github.com/{}/commit/{})",
        short_sha, options.repository, commit_sha
    ));

    Ok(format!("- {}\n", title))
}

fn get_file_creation_commit(file_path: &Path, options: &Options) -> Result<(String, String)> {
    // Execute the git log --follow command
    let output = Command::new("git")
        .args([
            "log",
            "--follow",
            "--diff-filter=A",       // Only show commits where the file was added
            "--pretty=format:%H %s", // Include both the commit SHA and the commit message
            "--",
            file_path.to_str().unwrap(),
        ])
        .current_dir(options.repository_path)
        .output()
        .context("Failed to execute git log command")?;

    // Check if the command was successful
    if !output.status.success() {
        bail!(
            "Git command failed [{}]",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Convert the output to a string
    let output_str = std::str::from_utf8(&output.stdout)?.trim();

    if output_str.is_empty() {
        bail!(
            "No commit found for the given file [{}]",
            file_path.display()
        );
    }

    // Split the output into commit SHA and commit message
    let mut parts = output_str.splitn(2, ' ');
    let commit_sha = parts
        .next()
        .ok_or(format_err!("Failed to parse commit SHA"))?
        .to_string();
    let commit_message = parts
        .next()
        .ok_or(format_err!("Failed to parse commit message"))?
        .to_string();

    Ok((commit_sha, commit_message))
}

struct ChangelogEntryWithPath {
    entry: ChangelogEntry,
    path: PathBuf,
}
