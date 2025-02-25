use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Context;
use tempfile::{tempdir, TempDir};

#[test]
fn test() {

    let repository = Repository::new();

    repository
        .copy_file("tests/example/.changelog/0.1.0/1_added.yaml")
        .add_and_commit("Add 1_added.yaml")
        .print_log();

}


struct Repository {
    dir: TempDir
}

impl Repository {
    fn new() -> Self {
        let temp_dir = tempdir().unwrap();
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        Repository { dir: temp_dir }
    }

    fn copy_file(self, file_name: &str) -> Self {
        let source = PathBuf::from(file_name);
        let destination = self.dir.path().join(file_name);
        let parent_dir = destination.parent().unwrap();
        std::fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create directory {}", parent_dir.display()))
            .unwrap();
        std::fs::copy(&source, &destination)
            .with_context(|| format!("Failed to copy {} to {}", source.display(), destination.display()))
            .unwrap();
        self
    }

    fn add_and_commit(self, message: &str) -> Self {
        let output = std::process::Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.dir.path())
            .envs(self.deterministic_envs())
            .output()
            .unwrap();

        if (!output.status.success()) {
            // println!("Error: {:?}", output);
            panic!("Error: {:?}", output);
        }

        // assert!(output.status.success());
        let output = std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(self.dir.path())
            .envs(self.deterministic_envs())
            .output()
            .unwrap();
        assert!(output.status.success());
        self
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