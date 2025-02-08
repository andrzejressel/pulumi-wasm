#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPlanRule {
    #[builder(into)]
    #[serde(rename = "completionWindow")]
    pub r#completion_window: Box<i32>,
    #[builder(into)]
    #[serde(rename = "copyActions")]
    pub r#copy_actions: Box<Vec<super::super::types::backup::GetPlanRuleCopyAction>>,
    #[builder(into)]
    #[serde(rename = "enableContinuousBackup")]
    pub r#enable_continuous_backup: Box<bool>,
    #[builder(into)]
    #[serde(rename = "lifecycles")]
    pub r#lifecycles: Box<Vec<super::super::types::backup::GetPlanRuleLifecycle>>,
    #[builder(into, default)]
    #[serde(rename = "recoveryPointTags")]
    pub r#recovery_point_tags: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<String>,
    #[builder(into)]
    #[serde(rename = "scheduleExpressionTimezone")]
    pub r#schedule_expression_timezone: Box<String>,
    #[builder(into)]
    #[serde(rename = "startWindow")]
    pub r#start_window: Box<i32>,
    #[builder(into)]
    #[serde(rename = "targetVaultName")]
    pub r#target_vault_name: Box<String>,
}
