#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceBackupConfigInfo {
    /// Configuration for an application backed up by a Backup Appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceBackupConfigs")]
    pub r#backup_appliance_backup_configs: Box<Vec<super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfoBackupApplianceBackupConfig>>,
    /// Configuration for a Google Cloud resource.
    #[builder(into)]
    #[serde(rename = "gcpBackupConfigs")]
    pub r#gcp_backup_configs: Box<Vec<super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfoGcpBackupConfig>>,
    /// If the last backup failed, this field has the error message.
    #[builder(into)]
    #[serde(rename = "lastBackupError")]
    pub r#last_backup_error: Box<std::collections::HashMap<String, String>>,
    /// LastBackupstate tracks whether the last backup was not yet started, successful, failed, or could not be run because of the lack of permissions.
    #[builder(into)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: Box<String>,
    /// If the last backup were successful, this field has the consistency date.
    #[builder(into)]
    #[serde(rename = "lastSuccessfulBackupConsistencyTime")]
    pub r#last_successful_backup_consistency_time: Box<String>,
}
