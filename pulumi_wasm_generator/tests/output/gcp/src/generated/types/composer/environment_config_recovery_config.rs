#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigRecoveryConfig {
    /// The configuration settings for scheduled snapshots.
    #[builder(into, default)]
    #[serde(rename = "scheduledSnapshotsConfig")]
    pub r#scheduled_snapshots_config: Box<Option<super::super::types::composer::EnvironmentConfigRecoveryConfigScheduledSnapshotsConfig>>,
}
