use std::collections::HashMap;
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
        .add_and_commit("Add 6_security.yaml")?;

    changelog_lib::generate_changelog("test", &repository.dir.path())?;

    Ok(())

}


struct Repository {
    dir: TempDir
}

impl Repository {
    fn new() -> Result<Self> {
        let temp_dir = tempdir().unwrap();
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
        std::fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create directory {}", parent_dir.display()))?;
        std::fs::copy(&source, &destination)
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

        // assert!(output.status.success());
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
}