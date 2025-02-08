#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DenyPolicyRule {
    /// A deny rule in an IAM deny policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "denyRule")]
    pub r#deny_rule: Box<Option<super::super::types::iam::DenyPolicyRuleDenyRule>>,
    /// The description of the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
}
