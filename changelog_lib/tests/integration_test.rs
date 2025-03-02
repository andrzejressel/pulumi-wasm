use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

fn replace_files() -> bool {
    false
}

const INITIAL_COMMIT_ID: &str = "145c01c05387eaa60cf2bfa61a2e81b11c6ef57d";

#[test]
fn generate_changelog_test() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog",
    };

    let result = changelog_lib::generate_changelog(&options)?;

    compare_with_file(&result, Path::new("tests/example/expected.md"))?;

    Ok(())
}

#[test]
fn generate_changelog_for_new_version() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog",
    };

    let result = changelog_lib::generate_changelog_for_new_version(&options, "0.3.0")?;

    compare_with_file(&result, Path::new("tests/example/expected_new_version.md"))?;

    Ok(())
}

#[test]
fn generate_github_changelog_for_0_1_0() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog",
    };

    let result = changelog_lib::generate_changelog_for_github_changelog(&options, "0.1.0")?;

    compare_with_file(&result, Path::new("tests/example/expected_github_0.1.0.md"))?;

    Ok(())
}

#[test]
fn generate_github_changelog_for_0_2_0() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog",
    };

    let result = changelog_lib::generate_changelog_for_github_changelog(&options, "0.2.0")?;

    compare_with_file(&result, Path::new("tests/example/expected_github_0.2.0.md"))?;

    Ok(())
}

#[test]
fn generate_mkdocs_changelog() -> Result<()> {
    let repository = create_repository().context("Failed to create repository")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: INITIAL_COMMIT_ID,
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog",
    };

    let result = changelog_lib::generate_mkdocs_changelog(&options)?;

    compare_with_file(&result, Path::new("tests/example/expected_mkdocs.md"))?;

    Ok(())
}

fn compare_with_file(content: &str, file: &Path) -> Result<()> {
    if replace_files() {
        fs::write(file, content)?;
    }
    let expected = fs::read(file).with_context(|| format!("Failed to read {}", file.display()))?;
    let expected = String::from_utf8(expected).context("Failed to convert to string")?;
    assert_eq!(content, expected);
    Ok(())
}

fn create_repository() -> Result<Repository> {
    let repository = Repository::new()?;

    let repository = repository
        .copy_file("tests/example/.changelog/0.1.0/.gitkeep")?
        .add_and_commit("Initial commit (MUST NOT BE INCLUDED)")?
        .copy_file("tests/example/.changelog/0.1.0/1_added.yaml")?
        .add_and_commit("Add 1_added.yaml")?
        .copy_file("tests/example/.changelog/0.1.0/2_changed.yaml")?
        .add_and_commit("Add 2_changed.yaml")?
        .copy_file("tests/example/.changelog/0.1.0/3_deprecated.yaml")?
        .add_and_commit("Add 3_deprecated.yaml")?
        .copy_file("tests/example/.changelog/0.1.0/4_removed.yaml")?
        .add_and_commit("Add 4_removed.yaml")?
        .copy_file("tests/example/.changelog/0.1.0/5_fixed.yaml")?
        .add_and_commit("Add 5_fixed.yaml")?
        .copy_file("tests/example/.changelog/0.1.0/6_security.yaml")?
        .add_and_commit("Add 6_security.yaml")?
        .create_tag("v0.1.0")?
        .add_and_commit("[no-changelog] Do not include in changelog")?
        .add_and_commit("Some feature")?
        .add_and_commit_renovate("Some renovate bot commit")?
        .create_tag("v0.2.0")?
        .copy_file("tests/example/.changelog/unreleased/1_added.yaml")?
        .add_and_commit("Some yet unreleased feature")?;

    Ok(repository)
}

struct Repository {
    dir: TempDir,
}

impl Repository {
    fn new() -> Result<Self> {
        let temp_dir = tempdir().context("Failed to create temporary directory")?;
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to initialize git repository")?;
        Ok(Repository { dir: temp_dir })
    }

    fn copy_file(self, file_name: &str) -> Result<Self> {
        let source = PathBuf::from(file_name);
        let destination = self.dir.path().join(file_name);
        let parent_dir = destination.parent().unwrap();
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create directory {}", parent_dir.display()))?;
        fs::copy(&source, &destination).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                source.display(),
                destination.display()
            )
        })?;
        Ok(self)
    }

    fn add_and_commit(self, message: &str) -> Result<Self> {
        let output = std::process::Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.dir.path())
            .envs(self.deterministic_envs())
            .output()
            .context("Failed to add")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        let output = std::process::Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg(message)
            .current_dir(self.dir.path())
            .envs(self.deterministic_envs())
            .output()
            .context("Failed to commit")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        Ok(self)
    }

    fn add_and_commit_renovate(self, message: &str) -> Result<Self> {
        let output = std::process::Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.dir.path())
            .envs(self.renovate_bot_envs())
            .output()
            .context("Failed to add")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        let output = std::process::Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg(message)
            .current_dir(self.dir.path())
            .envs(self.renovate_bot_envs())
            .output()
            .context("Failed to commit")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        Ok(self)
    }

    fn create_tag(self, tag_name: &str) -> Result<Self> {
        let output = std::process::Command::new("git")
            .arg("tag")
            .arg(tag_name)
            .current_dir(self.dir.path())
            .output()
            .context("Failed to create tag")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        Ok(self)
    }

    fn deterministic_envs(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();

        // Insert the environment variables into the HashMap
        env_vars.insert("GIT_AUTHOR_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_AUTHOR_EMAIL".to_string(),
            "your.email@example.com".to_string(),
        );
        env_vars.insert("GIT_COMMITTER_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_COMMITTER_EMAIL".to_string(),
            "your.email@example.com".to_string(),
        );
        env_vars.insert(
            "GIT_AUTHOR_DATE".to_string(),
            "2023-10-01T12:00:00+0000".to_string(),
        );
        env_vars.insert(
            "GIT_COMMITTER_DATE".to_string(),
            "2023-10-01T12:00:00+0000".to_string(),
        );

        env_vars
    }

    fn renovate_bot_envs(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        env_vars.insert("GIT_AUTHOR_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_AUTHOR_EMAIL".to_string(),
            "29139614+renovate[bot]@users.noreply.github.com".to_string(),
        );
        env_vars.insert("GIT_COMMITTER_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_COMMITTER_EMAIL".to_string(),
            "29139614+renovate[bot]@users.noreply.github.com".to_string(),
        );
        env_vars.insert(
            "GIT_AUTHOR_DATE".to_string(),
            "2023-10-01T12:00:00+0000".to_string(),
        );
        env_vars.insert(
            "GIT_COMMITTER_DATE".to_string(),
            "2023-10-01T12:00:00+0000".to_string(),
        );

        env_vars
    }
}
