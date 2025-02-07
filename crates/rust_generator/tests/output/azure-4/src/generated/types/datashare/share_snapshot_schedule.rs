#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ShareSnapshotSchedule {
    /// The name of the snapshot schedule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The interval of the synchronization with the source data. Possible values are `Hour` and `Day`.
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<String>,
    /// The synchronization with the source data's start time.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
