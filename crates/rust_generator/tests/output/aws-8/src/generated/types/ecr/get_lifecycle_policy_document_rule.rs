#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetLifecyclePolicyDocumentRule {
    /// Specifies the action type.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::ecr::GetLifecyclePolicyDocumentRuleAction>>,
    /// Describes the purpose of a rule within a lifecycle policy.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Sets the order in which rules are evaluated, lowest to highest. When you add rules to a lifecycle policy, you must give them each a unique value for `priority`. Values do not need to be sequential across rules in a policy. A rule with a `tag_status` value of "any" must have the highest value for `priority` and be evaluated last.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Collects parameters describing the selection criteria for the ECR lifecycle policy:
    #[builder(into, default)]
    #[serde(rename = "selection")]
    pub r#selection: Box<Option<super::super::types::ecr::GetLifecyclePolicyDocumentRuleSelection>>,
}
