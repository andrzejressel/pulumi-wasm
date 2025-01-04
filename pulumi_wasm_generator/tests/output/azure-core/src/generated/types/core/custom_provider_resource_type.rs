#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomProviderResourceType {
    /// Specifies the endpoint of the route definition.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
    /// Specifies the name of the route definition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The routing type that is supported for the resource request. Valid values are `Proxy` and `Proxy,Cache`. Defaults to `Proxy`.
    #[builder(into, default)]
    #[serde(rename = "routingType")]
    pub r#routing_type: Box<Option<String>>,
}
