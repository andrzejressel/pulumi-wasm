#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RulesEngineRuleAction {
    /// A `request_header` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Box<Option<Vec<super::super::types::frontdoor::RulesEngineRuleActionRequestHeader>>>,
    /// A `response_header` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Box<Option<Vec<super::super::types::frontdoor::RulesEngineRuleActionResponseHeader>>>,
}
