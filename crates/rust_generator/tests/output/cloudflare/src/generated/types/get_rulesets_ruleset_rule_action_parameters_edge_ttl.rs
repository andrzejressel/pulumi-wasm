#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtl {
    /// Default edge TTL
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the edge TTL.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[builder(into, default)]
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls: Box<Option<Vec<super::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
