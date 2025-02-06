#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupConfig {
    /// If True, include all namespaced resources.
    #[builder(into, default)]
    #[serde(rename = "allNamespaces")]
    pub r#all_namespaces: Box<Option<bool>>,
    /// This defines a customer managed encryption key that will be used to encrypt the "config"
    /// portion (the Kubernetes resources) of Backups created via this plan.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<Option<super::super::types::gkebackup::BackupPlanBackupConfigEncryptionKey>>,
    /// This flag specifies whether Kubernetes Secret resources should be included
    /// when they fall into the scope of Backups.
    #[builder(into, default)]
    #[serde(rename = "includeSecrets")]
    pub r#include_secrets: Box<Option<bool>>,
    /// This flag specifies whether volume data should be backed up when PVCs are
    /// included in the scope of a Backup.
    #[builder(into, default)]
    #[serde(rename = "includeVolumeData")]
    pub r#include_volume_data: Box<Option<bool>>,
    /// This flag specifies whether Backups will not fail when
    /// Backup for GKE detects Kubernetes configuration that is
    /// non-standard or requires additional setup to restore.
    #[builder(into, default)]
    #[serde(rename = "permissiveMode")]
    pub r#permissive_mode: Box<Option<bool>>,
    /// A list of namespaced Kubernetes Resources.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "selectedApplications")]
    pub r#selected_applications: Box<Option<super::super::types::gkebackup::BackupPlanBackupConfigSelectedApplications>>,
    /// If set, include just the resources in the listed namespaces.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "selectedNamespaces")]
    pub r#selected_namespaces: Box<Option<super::super::types::gkebackup::BackupPlanBackupConfigSelectedNamespaces>>,
}
