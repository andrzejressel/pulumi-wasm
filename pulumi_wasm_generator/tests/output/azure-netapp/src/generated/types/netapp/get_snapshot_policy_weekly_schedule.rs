#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSnapshotPolicyWeeklySchedule {
    /// List of the week days using English names when the snapshots will be created.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// Hour of the day that the snapshots will be created.
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: Box<i32>,
    /// Minute of the hour that the snapshots will be created.
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: Box<i32>,
    /// How many hourly snapshots to keep.
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: Box<i32>,
}
