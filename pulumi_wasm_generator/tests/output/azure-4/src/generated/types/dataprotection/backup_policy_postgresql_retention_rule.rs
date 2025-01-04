#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPolicyPostgresqlRetentionRule {
    /// A `criteria` block as defined below. Changing this forces a new Backup Policy PostgreSQL to be created.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: Box<super::super::types::dataprotection::BackupPolicyPostgresqlRetentionRuleCriteria>,
    /// Duration after which the backup is deleted. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy PostgreSQL to be created.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// The name which should be used for this retention rule. Changing this forces a new Backup Policy PostgreSQL to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the priority of the rule. The priority number must be unique for each rule. The lower the priority number, the higher the priority of the rule. Changing this forces a new Backup Policy PostgreSQL to be created.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
