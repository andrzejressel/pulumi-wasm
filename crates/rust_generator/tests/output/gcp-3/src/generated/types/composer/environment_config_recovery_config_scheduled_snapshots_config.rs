#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigRecoveryConfigScheduledSnapshotsConfig {
    /// When enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Snapshot schedule, in the unix-cron format.
    #[builder(into, default)]
    #[serde(rename = "snapshotCreationSchedule")]
    pub r#snapshot_creation_schedule: Box<Option<String>>,
    /// the URI of a bucket folder where to save the snapshot.
    #[builder(into, default)]
    #[serde(rename = "snapshotLocation")]
    pub r#snapshot_location: Box<Option<String>>,
    /// A time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
