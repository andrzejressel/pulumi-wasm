use crate::model::TagNameWithVersion::{NotYetReleasedWithVersion, RealTag};
use serde::Deserialize;
use std::path::PathBuf;
use TagName::{NotYetReleased, WithVersion};

#[derive(Debug, Deserialize)]
pub(crate) enum ChangelogType {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChangelogEntry {
    pub(crate) r#type: ChangelogType,
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) additional_pull_requests: Vec<i64>,
}

#[derive(Debug)]
pub(crate) struct GitHistory {
    pub(crate) versions: Vec<Version>,
}

#[derive(Clone, Debug)]
pub(crate) struct Version {
    pub(crate) tag_name: TagName,
    pub(crate) dependency_update_commits: Vec<Commit>,
    pub(crate) commits: Vec<Commit>,
}

#[derive(Clone, Debug)]
pub(crate) struct Commit {
    pub(crate) id: String,
    pub(crate) title: String,
}

#[derive(Clone, Debug)]
pub(crate) enum TagName {
    NotYetReleased,
    WithVersion(TagNameWithVersion),
}

#[derive(Clone, Debug)]
pub(crate) enum TagNameWithVersion {
    NotYetReleasedWithVersion(String),
    RealTag(String),
}

impl TagNameWithVersion {
    pub(crate) fn get_changelog_yaml_directory(&self) -> PathBuf {
        match self {
            NotYetReleasedWithVersion(_) => "unreleased".into(),
            RealTag(version) => strip_v(version).into(),
        }
    }

    pub(crate) fn get_tag(&self) -> String {
        match self {
            NotYetReleasedWithVersion(version) => format!("v{}", version),
            RealTag(version) => version.to_string(),
        }
    }

    pub(crate) fn get_version_name(&self) -> String {
        match self {
            NotYetReleasedWithVersion(version) => version.to_string(),
            RealTag(version) => strip_v(version).to_string(),
        }
    }
}

impl TagName {
    pub(crate) fn not_yet_released() -> TagName {
        NotYetReleased
    }

    pub(crate) fn not_yet_released_with_version(version: String) -> TagName {
        WithVersion(NotYetReleasedWithVersion(version))
    }

    pub(crate) fn real_tag(version: String) -> TagName {
        WithVersion(RealTag(version))
    }

    pub(crate) fn get_changelog_yaml_directory(&self) -> PathBuf {
        match self {
            NotYetReleased => "unreleased".into(),
            WithVersion(v) => v.get_changelog_yaml_directory(),
        }
    }

    pub(crate) fn get_version_name(&self) -> String {
        match self {
            NotYetReleased => "Unreleased".into(),
            WithVersion(v) => v.get_version_name(),
        }
    }

    pub(crate) fn get_tag(&self) -> String {
        match self {
            NotYetReleased => "HEAD".into(),
            WithVersion(v) => v.get_tag(),
        }
    }

    pub(crate) fn is_real_tag_with_version(&self, version: &str) -> bool {
        match self {
            WithVersion(RealTag(real_version)) => strip_v(real_version) == version,
            _ => false,
        }
    }
}

fn strip_v(version: &str) -> &str {
    version.strip_prefix("v").unwrap_or(version)
}
