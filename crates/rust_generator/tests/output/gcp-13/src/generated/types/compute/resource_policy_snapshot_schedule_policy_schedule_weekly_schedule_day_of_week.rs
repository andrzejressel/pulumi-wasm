#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourcePolicySnapshotSchedulePolicyScheduleWeeklyScheduleDayOfWeek {
    /// The day of the week to create the snapshot. e.g. MONDAY
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// Time within the window to start the operations.
    /// It must be in format "HH:MM", where HH : [00-23] and MM : [00-00] GMT.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
