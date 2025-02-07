#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeSnapshotPolicyMonthlySchedule {
    /// Set the day or days of the month to make a snapshot (1-31). Accepts a comma separated number of days. Defaults to '1'.
    #[builder(into, default)]
    #[serde(rename = "daysOfMonth")]
    pub r#days_of_month: Box<Option<String>>,
    /// Set the hour to create the snapshot (0-23), defaults to midnight (0).
    #[builder(into, default)]
    #[serde(rename = "hour")]
    pub r#hour: Box<Option<i32>>,
    /// Set the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0).
    #[builder(into, default)]
    #[serde(rename = "minute")]
    pub r#minute: Box<Option<i32>>,
    /// The maximum number of snapshots to keep for the monthly schedule
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: Box<i32>,
}
