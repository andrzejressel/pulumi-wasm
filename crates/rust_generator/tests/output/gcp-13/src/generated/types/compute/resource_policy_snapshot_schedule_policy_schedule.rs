#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourcePolicySnapshotSchedulePolicySchedule {
    /// The policy will execute every nth day at the specified time.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Box<Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleDailySchedule>>,
    /// The policy will execute every nth hour starting at the specified time.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hourlySchedule")]
    pub r#hourly_schedule: Box<Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleHourlySchedule>>,
    /// Allows specifying a snapshot time for each day of the week.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Box<Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleWeeklySchedule>>,
}
