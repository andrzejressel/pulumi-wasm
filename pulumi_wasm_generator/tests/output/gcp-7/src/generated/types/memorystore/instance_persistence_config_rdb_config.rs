#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePersistenceConfigRdbConfig {
    /// Optional. Period between RDB snapshots.
    /// Possible values:
    /// ONE_HOUR
    /// SIX_HOURS
    /// TWELVE_HOURS
    /// TWENTY_FOUR_HOURS
    #[builder(into, default)]
    #[serde(rename = "rdbSnapshotPeriod")]
    pub r#rdb_snapshot_period: Box<Option<String>>,
    /// Optional. Time that the first snapshot was/will be attempted, and to which future
    /// snapshots will be aligned. If not provided, the current time will be
    /// used.
    #[builder(into, default)]
    #[serde(rename = "rdbSnapshotStartTime")]
    pub r#rdb_snapshot_start_time: Box<Option<String>>,
}
