#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPolicyKubernetesClusterRetentionRule {
    /// A `criteria` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: Box<super::super::types::dataprotection::BackupPolicyKubernetesClusterRetentionRuleCriteria>,
    /// A `life_cycle` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "lifeCycles")]
    pub r#life_cycles: Box<Vec<super::super::types::dataprotection::BackupPolicyKubernetesClusterRetentionRuleLifeCycle>>,
    /// The name which should be used for this retention rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the priority of the rule. The priority number must be unique for each rule. The lower the priority number, the higher the priority of the rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
