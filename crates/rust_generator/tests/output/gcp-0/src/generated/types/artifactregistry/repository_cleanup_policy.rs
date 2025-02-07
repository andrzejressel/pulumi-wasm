#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryCleanupPolicy {
    /// Policy action.
    /// Possible values are: `DELETE`, `KEEP`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Policy condition for matching versions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::artifactregistry::RepositoryCleanupPolicyCondition>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Policy condition for retaining a minimum number of versions. May only be
    /// specified with a Keep action.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mostRecentVersions")]
    pub r#most_recent_versions: Box<Option<super::super::types::artifactregistry::RepositoryCleanupPolicyMostRecentVersions>>,
}
