#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessBoundaryPolicyRule {
    /// An access boundary rule in an IAM policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "accessBoundaryRule")]
    pub r#access_boundary_rule: Box<Option<super::super::types::iam::AccessBoundaryPolicyRuleAccessBoundaryRule>>,
    /// The description of the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
}
