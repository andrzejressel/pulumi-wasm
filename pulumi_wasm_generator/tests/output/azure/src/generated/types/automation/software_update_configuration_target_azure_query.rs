#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SoftwareUpdateConfigurationTargetAzureQuery {
    /// Specifies a list of locations to scope the query to.
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<String>>>,
    /// Specifies a list of Subscription or Resource Group ARM Ids to query.
    #[builder(into, default)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    /// Specifies how the specified tags to filter VMs. Possible values are `Any` and `All`.
    #[builder(into, default)]
    #[serde(rename = "tagFilter")]
    pub r#tag_filter: Box<Option<String>>,
    /// A mapping of tags used for query filter. One or more `tags` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetAzureQueryTag>>>,
}
