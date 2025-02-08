#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataSourceBackupConfigInfoGcpBackupConfig {
    /// The name of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlan")]
    pub r#backup_plan: Box<String>,
    /// The name of the backup plan association.
    #[builder(into)]
    #[serde(rename = "backupPlanAssociation")]
    pub r#backup_plan_association: Box<String>,
    /// The description of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlanDescription")]
    pub r#backup_plan_description: Box<String>,
    /// The names of the backup plan rules which point to this backupvault
    #[builder(into)]
    #[serde(rename = "backupPlanRules")]
    pub r#backup_plan_rules: Box<Vec<String>>,
}
