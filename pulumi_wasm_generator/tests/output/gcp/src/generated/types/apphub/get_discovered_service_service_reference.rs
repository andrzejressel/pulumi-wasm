#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDiscoveredServiceServiceReference {
    /// Additional path under the resource URI.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The underlying resource URI.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
