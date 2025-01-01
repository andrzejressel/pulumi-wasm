#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAutomatedBackupPolicy {
    /// The length of the time window during which a backup can be taken. If a backup does not succeed within this time window, it will be canceled and considered failed.
    /// The backup window must be at least 5 minutes long. There is no upper bound on the window. If not set, it will default to 1 hour.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "backupWindow")]
    pub r#backup_window: Box<Option<String>>,
    /// Whether automated backups are enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<Option<super::super::types::alloydb::ClusterAutomatedBackupPolicyEncryptionConfig>>,
    /// Labels to apply to backups created using this configuration.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The location where the backup will be stored. Currently, the only supported option is to store the backup in the same region as the cluster.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Quantity-based Backup retention policy to retain recent backups. Conflicts with 'time_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "quantityBasedRetention")]
    pub r#quantity_based_retention: Box<Option<super::super::types::alloydb::ClusterAutomatedBackupPolicyQuantityBasedRetention>>,
    /// Time-based Backup retention policy. Conflicts with 'quantity_based_retention', both can't be set together.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timeBasedRetention")]
    pub r#time_based_retention: Box<Option<super::super::types::alloydb::ClusterAutomatedBackupPolicyTimeBasedRetention>>,
    /// Weekly schedule for the Backup.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Box<Option<super::super::types::alloydb::ClusterAutomatedBackupPolicyWeeklySchedule>>,
}
