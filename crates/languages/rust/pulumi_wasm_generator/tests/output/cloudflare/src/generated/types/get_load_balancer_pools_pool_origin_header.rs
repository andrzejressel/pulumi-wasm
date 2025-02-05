#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    /// HTTP Header name.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
