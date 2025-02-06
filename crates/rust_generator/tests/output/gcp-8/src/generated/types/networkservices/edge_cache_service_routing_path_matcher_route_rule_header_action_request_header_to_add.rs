#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToAdd {
    /// The name of the header to add.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
    /// The value of the header to add.
    #[builder(into)]
    #[serde(rename = "headerValue")]
    pub r#header_value: Box<String>,
    /// Whether to replace all existing headers with the same name.
    #[builder(into, default)]
    #[serde(rename = "replace")]
    pub r#replace: Box<Option<bool>>,
}
