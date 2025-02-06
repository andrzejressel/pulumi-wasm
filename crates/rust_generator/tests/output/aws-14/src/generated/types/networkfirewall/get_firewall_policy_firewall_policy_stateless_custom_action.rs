#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallPolicyFirewallPolicyStatelessCustomAction {
    #[builder(into)]
    #[serde(rename = "actionDefinitions")]
    pub r#action_definitions: Box<Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition>>,
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: Box<String>,
}
