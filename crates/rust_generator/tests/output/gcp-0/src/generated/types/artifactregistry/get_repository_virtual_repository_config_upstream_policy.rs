#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryVirtualRepositoryConfigUpstreamPolicy {
    /// The user-provided ID of the upstream policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Entries with a greater priority value take precedence in the pull order.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// A reference to the repository resource, for example:
    /// "projects/p1/locations/us-central1/repository/repo1".
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
}
