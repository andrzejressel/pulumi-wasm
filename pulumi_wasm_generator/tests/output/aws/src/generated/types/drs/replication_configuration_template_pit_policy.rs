#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigurationTemplatePitPolicy {
    /// Whether this rule is enabled or not.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// How often, in the chosen units, a snapshot should be taken.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// Duration to retain a snapshot for, in the chosen `units`.
    #[builder(into)]
    #[serde(rename = "retentionDuration")]
    pub r#retention_duration: Box<i32>,
    /// ID of the rule. Valid values are integers.
    #[builder(into, default)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<Option<i32>>,
    /// Units used to measure the `interval` and `retention_duration`. Valid values are `MINUTE`, `HOUR`, and `DAY`.
    #[builder(into)]
    #[serde(rename = "units")]
    pub r#units: Box<String>,
}