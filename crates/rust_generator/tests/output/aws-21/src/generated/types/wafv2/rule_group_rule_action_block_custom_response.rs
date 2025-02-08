#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleActionBlockCustomResponse {
    /// References the response body that you want AWS WAF to return to the web request client. This must reference a `key` defined in a `custom_response_body` block of this resource.
    #[builder(into, default)]
    #[serde(rename = "customResponseBodyKey")]
    pub r#custom_response_body_key: Box<Option<String>>,
    /// The HTTP status code to return to the client.
    #[builder(into)]
    #[serde(rename = "responseCode")]
    pub r#response_code: Box<i32>,
    /// The `response_header` blocks used to define the HTTP response headers added to the response. See Custom HTTP Header below for details.
    #[builder(into, default)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Box<Option<Vec<super::super::types::wafv2::RuleGroupRuleActionBlockCustomResponseResponseHeader>>>,
}
