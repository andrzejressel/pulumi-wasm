#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleSourceCustomPolicyDetails {
    /// The boolean expression for enabling debug logging for your Config Custom Policy rule. The default value is `false`.
    #[builder(into, default)]
    #[serde(rename = "enableDebugLogDelivery")]
    pub r#enable_debug_log_delivery: Box<Option<bool>>,
    /// The runtime system for your Config Custom Policy rule. Guard is a policy-as-code language that allows you to write policies that are enforced by Config Custom Policy rules. For more information about Guard, see the [Guard GitHub Repository](https://github.com/aws-cloudformation/cloudformation-guard).
    #[builder(into)]
    #[serde(rename = "policyRuntime")]
    pub r#policy_runtime: Box<String>,
    /// The policy definition containing the logic for your Config Custom Policy rule.
    #[builder(into)]
    #[serde(rename = "policyText")]
    pub r#policy_text: Box<String>,
}
