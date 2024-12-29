#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleOverrideAction {
    /// Override the rule action setting to count (i.e., only count matches). Configured as an empty block `{}`.
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<super::super::types::wafv2::WebAclRuleOverrideActionCount>>,
    /// Don't override the rule action setting. Configured as an empty block `{}`.
    #[builder(into, default)]
    #[serde(rename = "none")]
    pub r#none: Box<Option<super::super::types::wafv2::WebAclRuleOverrideActionNone>>,
}
