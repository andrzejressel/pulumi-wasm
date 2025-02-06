#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OutboundConnectionConnectionPropertiesCrossClusterSearch {
    /// Skips unavailable clusters and can only be used for cross-cluster searches. Accepted values are `ENABLED` or `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "skipUnavailable")]
    pub r#skip_unavailable: Box<Option<String>>,
}
