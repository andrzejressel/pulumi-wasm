#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RepositoryCleanupPolicyMostRecentVersions {
    /// Minimum number of versions to keep.
    #[builder(into, default)]
    #[serde(rename = "keepCount")]
    pub r#keep_count: Box<Option<i32>>,
    /// Match versions by package prefix. Applied on any prefix match.
    #[builder(into, default)]
    #[serde(rename = "packageNamePrefixes")]
    pub r#package_name_prefixes: Box<Option<Vec<String>>>,
}
