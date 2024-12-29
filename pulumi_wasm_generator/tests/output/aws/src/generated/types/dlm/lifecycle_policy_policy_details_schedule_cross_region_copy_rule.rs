#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule {
    #[builder(into, default)]
    #[serde(rename = "cmkArn")]
    pub r#cmk_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "copyTags")]
    pub r#copy_tags: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "deprecateRule")]
    pub r#deprecate_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleDeprecateRule>>,
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<bool>,
    #[builder(into, default)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleRetainRule>>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}
