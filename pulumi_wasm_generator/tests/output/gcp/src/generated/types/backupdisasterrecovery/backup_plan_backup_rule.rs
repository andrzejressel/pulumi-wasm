#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupRule {
    /// Configures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault.
    #[builder(into)]
    #[serde(rename = "backupRetentionDays")]
    pub r#backup_retention_days: Box<i32>,
    /// The unique ID of this `BackupRule`. The `rule_id` is unique per `BackupPlan`.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<String>,
    /// StandardSchedule defines a schedule that runs within the confines of a defined window of days.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "standardSchedule")]
    pub r#standard_schedule: Box<super::super::types::backupdisasterrecovery::BackupPlanBackupRuleStandardSchedule>,
}
