#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PageRuleActionsCacheTtlByStatus {
    /// A HTTP code (e.g. `404`) or range of codes (e.g. `400-499`)
    #[builder(into)]
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    /// Duration a resource lives in the Cloudflare cache.
    /// - positive number - cache for specified duration in seconds
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}
