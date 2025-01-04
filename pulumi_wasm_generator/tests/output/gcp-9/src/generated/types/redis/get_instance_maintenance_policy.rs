#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceMaintenancePolicy {
    /// Output only. The time when the policy was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// Optional. Description of what this policy is for.
    /// Create/Update methods return INVALID_ARGUMENT if the
    /// length is greater than 512.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Output only. The time when the policy was last updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
    /// Optional. Maintenance window that is applied to resources covered by this policy.
    /// Minimum 1. For the current version, the maximum number
    /// of weekly_window is expected to be one.
    #[builder(into)]
    #[serde(rename = "weeklyMaintenanceWindows")]
    pub r#weekly_maintenance_windows: Box<Vec<super::super::types::redis::GetInstanceMaintenancePolicyWeeklyMaintenanceWindow>>,
}
