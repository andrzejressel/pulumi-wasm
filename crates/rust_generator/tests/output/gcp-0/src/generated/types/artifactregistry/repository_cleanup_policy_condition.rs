#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryCleanupPolicyCondition {
    /// Match versions newer than a duration.
    #[builder(into, default)]
    #[serde(rename = "newerThan")]
    pub r#newer_than: Box<Option<String>>,
    /// Match versions older than a duration.
    #[builder(into, default)]
    #[serde(rename = "olderThan")]
    pub r#older_than: Box<Option<String>>,
    /// Match versions by package prefix. Applied on any prefix match.
    #[builder(into, default)]
    #[serde(rename = "packageNamePrefixes")]
    pub r#package_name_prefixes: Box<Option<Vec<String>>>,
    /// Match versions by tag prefix. Applied on any prefix match.
    #[builder(into, default)]
    #[serde(rename = "tagPrefixes")]
    pub r#tag_prefixes: Box<Option<Vec<String>>>,
    /// Match versions by tag status.
    /// Default value is `ANY`.
    /// Possible values are: `TAGGED`, `UNTAGGED`, `ANY`.
    #[builder(into, default)]
    #[serde(rename = "tagState")]
    pub r#tag_state: Box<Option<String>>,
    /// Match versions by version name prefix. Applied on any prefix match.
    #[builder(into, default)]
    #[serde(rename = "versionNamePrefixes")]
    pub r#version_name_prefixes: Box<Option<Vec<String>>>,
}
