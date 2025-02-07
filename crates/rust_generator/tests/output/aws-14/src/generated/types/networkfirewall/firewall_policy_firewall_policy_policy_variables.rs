#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicyPolicyVariables {
    #[builder(into, default)]
    #[serde(rename = "ruleVariables")]
    pub r#rule_variables: Box<Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyPolicyVariablesRuleVariable>>>,
}
