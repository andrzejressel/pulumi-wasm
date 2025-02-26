use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use tempfile::{tempdir, TempDir};
use anyhow::Result;

#[test]
fn test() -> Result<()> {

    let repository = Repository::new()?;

    let repository = repository
        .add_and_commit("Initial commit")?
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
        .create_tag("0.1.0")?
        .add_and_commit("[no-changelog] Do not include in changelog")?
        .add_and_commit("Some feature")?
        .add_and_commit_renovate("Some renovate bot commit")?
        .create_tag("0.2.0")?
        .add_and_commit("Some other feature")?;

    let options = changelog_lib::Options {
        repository_path: repository.dir.path(),
        start_commit_id: "e6f61d90d87238305276618124d965b0aa750a06",
        repository: "andrzejressel/pulumi-gestalt",
        changelog_dir: "tests/example/.changelog"
    };

    let result = changelog_lib::generate_changelog(&options)?;

    let expected = fs::read("tests/example/expected.md")
        .context("Failed to read expected.md")?;
    let expected = String::from_utf8(expected)?.replace("\r\n", "\n");

    assert_eq!(result, expected);

    Ok(())

}


struct Repository {
    dir: TempDir
}

impl Repository {
    fn new() -> Result<Self> {
        let temp_dir = tempdir()
            .context("Failed to create temporary directory")?;
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to initialize git repository")?;
        Ok(Repository { dir: temp_dir })
    }

    fn get_dir(&self) -> &TempDir {
        &self.dir
    }

    fn copy_file(self, file_name: &str) -> Result<Self> {
        let source = PathBuf::from(file_name);
        let destination = self.dir.path().join(file_name);
        let parent_dir = destination.parent().unwrap();
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create directory {}", parent_dir.display()))?;
        fs::copy(&source, &destination)
            .with_context(|| format!("Failed to copy {} to {}", source.display(), destination.display()))?;
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

    fn print_log(self) -> Self {
        let output = std::process::Command::new("git")
            .arg("log")
            .current_dir(self.dir.path())
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
        self
    }

    fn deterministic_envs(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();

        // Insert the environment variables into the HashMap
        env_vars.insert("GIT_AUTHOR_NAME".to_string(), "Your Name".to_string());
        env_vars.insert("GIT_AUTHOR_EMAIL".to_string(), "your.email@example.com".to_string());
        env_vars.insert("GIT_COMMITTER_NAME".to_string(), "Your Name".to_string());
        env_vars.insert("GIT_COMMITTER_EMAIL".to_string(), "your.email@example.com".to_string());
        env_vars.insert("GIT_AUTHOR_DATE".to_string(), "2023-10-01T12:00:00+0000".to_string());
        env_vars.insert("GIT_COMMITTER_DATE".to_string(), "2023-10-01T12:00:00+0000".to_string());

        env_vars
    }

    fn renovate_bot_envs(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        env_vars.insert("GIT_AUTHOR_NAME".to_string(), "Your Name".to_string());
        env_vars.insert("GIT_AUTHOR_EMAIL".to_string(), "29139614+renovate[bot]@users.noreply.github.com".to_string());
        env_vars.insert("GIT_COMMITTER_NAME".to_string(), "Your Name".to_string());
        env_vars.insert("GIT_COMMITTER_EMAIL".to_string(), "29139614+renovate[bot]@users.noreply.github.com".to_string());
        env_vars.insert("GIT_AUTHOR_DATE".to_string(), "2023-10-01T12:00:00+0000".to_string());
        env_vars.insert("GIT_COMMITTER_DATE".to_string(), "2023-10-01T12:00:00+0000".to_string());

        env_vars
    }
}