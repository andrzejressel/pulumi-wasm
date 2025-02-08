#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceMaintenancePolicy {
    /// (Output)
    /// Output only. The time when the policy was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Optional. Description of what this policy is for.
    /// Create/Update methods return INVALID_ARGUMENT if the
    /// length is greater than 512.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// (Output)
    /// Output only. The time when the policy was last updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
    /// Optional. Maintenance window that is applied to resources covered by this policy.
    /// Minimum 1. For the current version, the maximum number
    /// of weekly_window is expected to be one.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weeklyMaintenanceWindows")]
    pub r#weekly_maintenance_windows: Box<Option<Vec<super::super::types::redis::InstanceMaintenancePolicyWeeklyMaintenanceWindow>>>,
}
