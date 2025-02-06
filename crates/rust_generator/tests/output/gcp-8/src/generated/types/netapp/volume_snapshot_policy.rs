#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeSnapshotPolicy {
    /// Daily schedule policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Box<Option<super::super::types::netapp::VolumeSnapshotPolicyDailySchedule>>,
    /// Enables automated snapshot creation according to defined schedule. Default is false.
    /// To disable automatic snapshot creation you have to remove the whole snapshot_policy block.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Hourly schedule policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hourlySchedule")]
    pub r#hourly_schedule: Box<Option<super::super::types::netapp::VolumeSnapshotPolicyHourlySchedule>>,
    /// Monthly schedule policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "monthlySchedule")]
    pub r#monthly_schedule: Box<Option<super::super::types::netapp::VolumeSnapshotPolicyMonthlySchedule>>,
    /// Weekly schedule policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Box<Option<super::super::types::netapp::VolumeSnapshotPolicyWeeklySchedule>>,
}
