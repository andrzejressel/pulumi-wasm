#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AssignmentDynamicScopeFilter {
    /// Specifies a list of locations to scope the query to.
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<String>>>,
    /// Specifies a list of allowed operating systems.
    #[builder(into, default)]
    #[serde(rename = "osTypes")]
    pub r#os_types: Box<Option<Vec<String>>>,
    /// Specifies a list of allowed resource groups.
    #[builder(into, default)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Box<Option<Vec<String>>>,
    /// Specifies a list of allowed resources.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<String>>>,
    /// Filter VMs by `Any` or `All` specified tags. Defaults to `Any`.
    #[builder(into, default)]
    #[serde(rename = "tagFilter")]
    pub r#tag_filter: Box<Option<String>>,
    /// A mapping of tags for the VM
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<super::super::types::maintenance::AssignmentDynamicScopeFilterTag>>>,
}
