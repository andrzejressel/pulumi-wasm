#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanAssociationRulesConfigInfo {
    /// (Output)
    /// google.rpc.Status object to store the last backup error
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "lastBackupErrors")]
    pub r#last_backup_errors: Box<Option<Vec<super::super::types::backupdisasterrecovery::BackupPlanAssociationRulesConfigInfoLastBackupError>>>,
    /// (Output)
    /// State of last backup taken.
    #[builder(into, default)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: Box<Option<String>>,
    /// (Output)
    /// Backup Rule id fetched from backup plan.
    #[builder(into, default)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<Option<String>>,
}
