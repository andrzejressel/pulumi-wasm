#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackupPlanAssociationRulesConfigInfo {
    /// google.rpc.Status object to store the last backup error
    #[builder(into)]
    #[serde(rename = "lastBackupErrors")]
    pub r#last_backup_errors: Box<Vec<super::super::types::backupdisasterrecovery::GetBackupPlanAssociationRulesConfigInfoLastBackupError>>,
    /// State of last backup taken.
    #[builder(into)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: Box<String>,
    /// Backup Rule id fetched from backup plan.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<String>,
}
