#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServerlessCacheEndpoint {
    /// The DNS hostname of the cache node.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The port number that the cache engine is listening on. Set as integer.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}