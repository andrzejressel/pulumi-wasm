#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterContinuousBackupConfig {
    /// Whether continuous backup recovery is enabled. If not set, defaults to true.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<Option<super::super::types::alloydb::ClusterContinuousBackupConfigEncryptionConfig>>,
    /// The numbers of days that are eligible to restore from using PITR. To support the entire recovery window, backups and logs are retained for one day more than the recovery window.
    /// If not set, defaults to 14 days.
    #[builder(into, default)]
    #[serde(rename = "recoveryWindowDays")]
    pub r#recovery_window_days: Box<Option<i32>>,
}
