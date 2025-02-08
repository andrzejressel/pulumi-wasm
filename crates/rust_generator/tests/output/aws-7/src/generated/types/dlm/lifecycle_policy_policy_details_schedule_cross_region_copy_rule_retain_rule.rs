#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleRetainRule {
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    #[builder(into)]
    #[serde(rename = "intervalUnit")]
    pub r#interval_unit: Box<String>,
}
