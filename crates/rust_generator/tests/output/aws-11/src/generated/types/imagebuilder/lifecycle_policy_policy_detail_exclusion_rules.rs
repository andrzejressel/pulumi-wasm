#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LifecyclePolicyPolicyDetailExclusionRules {
    /// Lists configuration values that apply to AMIs that Image Builder should exclude from the lifecycle action. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "amis")]
    pub r#amis: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmis>>,
    /// Contains a list of tags that Image Builder uses to skip lifecycle actions for Image Builder image resources that have them.
    #[builder(into, default)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Box<Option<std::collections::HashMap<String, String>>>,
}
