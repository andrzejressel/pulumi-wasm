#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryCleanupPolicyMostRecentVersion {
    /// Minimum number of versions to keep.
    #[builder(into)]
    #[serde(rename = "keepCount")]
    pub r#keep_count: Box<i32>,
    /// Match versions by package prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "packageNamePrefixes")]
    pub r#package_name_prefixes: Box<Vec<String>>,
}
