#![allow(dead_code)]

use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

pub(crate) fn replace_files() -> bool {
    false
}

pub(crate) fn compare_with_file(content: &str, file: &Path) -> Result<()> {
    if replace_files() {
        fs::write(file, content)?;
    }
    let expected = fs::read(file).with_context(|| format!("Failed to read {}", file.display()))?;
    let expected = String::from_utf8(expected).context("Failed to convert to string")?;
    assert_eq!(content, expected);
    Ok(())
}

pub(crate) struct Repository {
    pub dir: TempDir,
}

impl Repository {
    pub(crate) fn new() -> Result<Self> {
        let temp_dir = tempdir().context("Failed to create temporary directory")?;
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to initialize git repository")?;
        Ok(Repository { dir: temp_dir })
    }

    pub(crate) fn copy_file(self, file_name: &str) -> Result<Self> {
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

    pub(crate) fn move_file(self, source: &str, destination: &str) -> Result<Self> {
        let source = self.dir.path().join(source);
        let destination = self.dir.path().join(destination);
        fs::rename(&source, &destination).with_context(|| {
            format!(
                "Failed to move {} to {}",
                source.display(),
                destination.display()
            )
        })?;

        Ok(self)
    }

    pub(crate) fn add_and_commit(self, message: &str) -> Result<Self> {
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

    pub(crate) fn add_and_commit_renovate(self, message: &str) -> Result<Self> {
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

    pub(crate) fn add_and_commit_dependabot(self, message: &str) -> Result<Self> {
        let output = std::process::Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.dir.path())
            .envs(self.dependabot_bot_envs())
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
            .envs(self.dependabot_bot_envs())
            .output()
            .context("Failed to commit")?;

        if !output.status.success() {
            anyhow::bail!("Error: {:?}", output);
        }

        Ok(self)
    }

    pub(crate) fn create_tag(self, tag_name: &str) -> Result<Self> {
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

    fn dependabot_bot_envs(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        env_vars.insert("GIT_AUTHOR_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_AUTHOR_EMAIL".to_string(),
            "49699333+dependabot[bot]@users.noreply.github.com".to_string(),
        );
        env_vars.insert("GIT_COMMITTER_NAME".to_string(), "Your Name".to_string());
        env_vars.insert(
            "GIT_COMMITTER_EMAIL".to_string(),
            "49699333+dependabot[bot]@users.noreply.github.com".to_string(),
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
