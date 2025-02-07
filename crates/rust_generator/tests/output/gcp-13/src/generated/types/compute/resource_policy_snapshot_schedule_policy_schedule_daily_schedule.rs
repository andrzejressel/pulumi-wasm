#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourcePolicySnapshotSchedulePolicyScheduleDailySchedule {
    /// Defines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1.
    #[builder(into)]
    #[serde(rename = "daysInCycle")]
    pub r#days_in_cycle: Box<i32>,
    /// This must be in UTC format that resolves to one of
    /// 00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,
    /// both 13:00-5 and 08:00 are valid.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
