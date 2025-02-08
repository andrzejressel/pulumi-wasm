#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFirewallPolicyFirewallPolicyStatefulRuleGroupReference {
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulRuleGroupReferenceOverride>>>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<String>,
}
