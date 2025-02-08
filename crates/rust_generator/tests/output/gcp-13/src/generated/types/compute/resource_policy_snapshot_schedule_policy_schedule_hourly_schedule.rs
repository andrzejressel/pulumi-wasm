#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourcePolicySnapshotSchedulePolicyScheduleHourlySchedule {
    /// The number of hours between snapshots.
    #[builder(into)]
    #[serde(rename = "hoursInCycle")]
    pub r#hours_in_cycle: Box<i32>,
    /// Time within the window to start the operations.
    /// It must be in an hourly format "HH:MM",
    /// where HH : [00-23] and MM : [00] GMT. eg: 21:00
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
