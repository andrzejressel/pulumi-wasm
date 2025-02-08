#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRepositoryCleanupPolicy {
    /// Policy action. Possible values: ["DELETE", "KEEP"]
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Policy condition for matching versions.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::artifactregistry::GetRepositoryCleanupPolicyCondition>>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Policy condition for retaining a minimum number of versions. May only be
    /// specified with a Keep action.
    #[builder(into)]
    #[serde(rename = "mostRecentVersions")]
    pub r#most_recent_versions: Box<Vec<super::super::types::artifactregistry::GetRepositoryCleanupPolicyMostRecentVersion>>,
}
