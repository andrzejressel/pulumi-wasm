#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigRecoveryConfig {
    /// The configuration settings for scheduled snapshots.
    #[builder(into)]
    #[serde(rename = "scheduledSnapshotsConfigs")]
    pub r#scheduled_snapshots_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigRecoveryConfigScheduledSnapshotsConfig>>,
}
