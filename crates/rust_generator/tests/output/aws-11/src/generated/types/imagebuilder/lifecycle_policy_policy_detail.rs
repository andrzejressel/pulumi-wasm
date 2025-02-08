#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetail {
    /// Configuration details for the policy action.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailAction>>,
    /// Additional rules to specify resources that should be exempt from policy actions.
    #[builder(into, default)]
    #[serde(rename = "exclusionRules")]
    pub r#exclusion_rules: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRules>>,
    /// Specifies the resources that the lifecycle policy applies to.
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailFilter>>,
}
