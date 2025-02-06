#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResourcePolicySnapshotSchedulePolicySchedule {
    /// The policy will execute every nth day at the specified time.
    #[builder(into)]
    #[serde(rename = "dailySchedules")]
    pub r#daily_schedules: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleDailySchedule>>,
    /// The policy will execute every nth hour starting at the specified time.
    #[builder(into)]
    #[serde(rename = "hourlySchedules")]
    pub r#hourly_schedules: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleHourlySchedule>>,
    /// Allows specifying a snapshot time for each day of the week.
    #[builder(into)]
    #[serde(rename = "weeklySchedules")]
    pub r#weekly_schedules: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyScheduleWeeklySchedule>>,
}
