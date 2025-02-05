#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryVirtualRepositoryConfigUpstreamPolicy {
    /// The user-provided ID of the upstream policy.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Entries with a greater priority value take precedence in the pull order.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// A reference to the repository resource, for example:
    /// "projects/p1/locations/us-central1/repository/repo1".
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<String>>,
}
