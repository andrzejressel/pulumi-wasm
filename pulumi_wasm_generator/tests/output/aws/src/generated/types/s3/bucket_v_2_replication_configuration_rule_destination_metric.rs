#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2ReplicationConfigurationRuleDestinationMetric {
    /// Threshold within which objects are to be replicated. The only valid value is `15`.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
    /// Status of replication metrics. Either `Enabled` or `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
