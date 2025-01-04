#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupPolicyAssignmentResourceSelector {
    /// Specifies a name for the resource selector.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// One or more `resource_selector` block as defined below.
    #[builder(into)]
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Vec<super::super::types::management::GroupPolicyAssignmentResourceSelectorSelector>>,
}
