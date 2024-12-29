#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationApplicationSnapshotConfiguration {
    /// Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.
    #[builder(into)]
    #[serde(rename = "snapshotsEnabled")]
    pub r#snapshots_enabled: Box<bool>,
}
