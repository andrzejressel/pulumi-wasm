#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PrincipalAccessBoundaryPolicyDetailsRule {
    /// The description of the principal access boundary policy rule. Must be less than or equal to 256 characters.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The access relationship of principals to the resources in this rule.
    /// Possible values: ALLOW
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Box<String>,
    /// A list of Cloud Resource Manager resources. The resource
    /// and all the descendants are included. The number of resources in a policy
    /// is limited to 500 across all rules.
    /// The following resource types are supported:
    /// * Organizations, such as `//cloudresourcemanager.googleapis.com/organizations/123`.
    /// * Folders, such as `//cloudresourcemanager.googleapis.com/folders/123`.
    /// * Projects, such as `//cloudresourcemanager.googleapis.com/projects/123`
    /// or `//cloudresourcemanager.googleapis.com/projects/my-project-id`.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Vec<String>>,
}
