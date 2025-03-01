use serde::Deserialize;
use crate::TagName;

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
}

#[derive(Debug)]
pub(crate) struct GitHistory {
    pub(crate) versions: Vec<Version>,
}

#[derive(Clone, Debug)]
pub(crate) struct Version {
    pub(crate) tag_name: Option<TagName>, // None for the unreleased version
    pub(crate) renovate_bot_commits: Vec<Commit>,
    pub(crate) commits: Vec<Commit>,
}

#[derive(Clone, Debug)]
pub(crate) struct Commit {
    pub(crate) id: String,
    pub(crate) title: String,
}
