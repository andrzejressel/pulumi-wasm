#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupRuleActionBlock {
    /// Defines a custom response for the web request. See Custom Response below for details.
    #[builder(into, default)]
    #[serde(rename = "customResponse")]
    pub r#custom_response: Box<Option<super::super::types::wafv2::RuleGroupRuleActionBlockCustomResponse>>,
}
