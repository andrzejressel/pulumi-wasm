#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SnapshotPolicyMonthlySchedule {
    /// List of the days of the month when the snapshots will be created, valid range is from 1 to 30.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Box<Vec<i32>>,
    /// Hour of the day that the snapshots will be created, valid range is from 0 to 23.
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: Box<i32>,
    /// Minute of the hour that the snapshots will be created, valid range is from 0 to 59.
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: Box<i32>,
    /// How many hourly snapshots to keep, valid range is from 0 to 255.
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: Box<i32>,
}
