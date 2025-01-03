#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAutomatedBackupPolicyTimeBasedRetention {
    /// The retention period.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Box<Option<String>>,
}
