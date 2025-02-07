#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstancePersistenceConfig {
    /// Optional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.
    /// 
    /// - DISABLED: 	Persistence is disabled for the instance, and any existing snapshots are deleted.
    /// - RDB: RDB based Persistence is enabled. Possible values: ["DISABLED", "RDB"]
    #[builder(into)]
    #[serde(rename = "persistenceMode")]
    pub r#persistence_mode: Box<String>,
    /// Output only. The next time that a snapshot attempt is scheduled to occur.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up
    /// to nine fractional digits.
    /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "rdbNextSnapshotTime")]
    pub r#rdb_next_snapshot_time: Box<String>,
    /// Optional. Available snapshot periods for scheduling.
    /// 
    /// - ONE_HOUR:	Snapshot every 1 hour.
    /// - SIX_HOURS:	Snapshot every 6 hours.
    /// - TWELVE_HOURS:	Snapshot every 12 hours.
    /// - TWENTY_FOUR_HOURS:	Snapshot every 24 hours. Possible values: ["ONE_HOUR", "SIX_HOURS", "TWELVE_HOURS", "TWENTY_FOUR_HOURS"]
    #[builder(into)]
    #[serde(rename = "rdbSnapshotPeriod")]
    pub r#rdb_snapshot_period: Box<String>,
    /// Optional. Date and time that the first snapshot was/will be attempted,
    /// and to which future snapshots will be aligned. If not provided,
    /// the current time will be used.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution
    /// and up to nine fractional digits.
    /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "rdbSnapshotStartTime")]
    pub r#rdb_snapshot_start_time: Box<String>,
}
