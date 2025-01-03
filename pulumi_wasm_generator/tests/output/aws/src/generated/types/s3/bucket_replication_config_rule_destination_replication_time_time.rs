#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketReplicationConfigRuleDestinationReplicationTimeTime {
    /// Time in minutes. Valid values: `15`.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<i32>,
}
