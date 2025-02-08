#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetEnvironmentConfigRecoveryConfigScheduledSnapshotsConfig {
    /// When enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Snapshot schedule, in the unix-cron format.
    #[builder(into)]
    #[serde(rename = "snapshotCreationSchedule")]
    pub r#snapshot_creation_schedule: Box<String>,
    /// the URI of a bucket folder where to save the snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotLocation")]
    pub r#snapshot_location: Box<String>,
    /// A time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
