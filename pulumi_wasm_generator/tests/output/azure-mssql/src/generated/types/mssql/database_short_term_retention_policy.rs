#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseShortTermRetentionPolicy {
    /// The hours between each differential backup. This is only applicable to live databases but not dropped databases. Value has to be `12` or `24`. Defaults to `12` hours.
    #[builder(into, default)]
    #[serde(rename = "backupIntervalInHours")]
    pub r#backup_interval_in_hours: Box<Option<i32>>,
    /// Point In Time Restore configuration. Value has to be between `1` and `35`.
    #[builder(into)]
    #[serde(rename = "retentionDays")]
    pub r#retention_days: Box<i32>,
}
