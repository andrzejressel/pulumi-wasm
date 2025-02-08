#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyRulePreconfiguredWafConfigExclusion {
    /// Request cookie whose value will be excluded from inspection during preconfigured WAF evaluation.
    #[builder(into)]
    #[serde(rename = "requestCookies")]
    pub r#request_cookies: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestCooky>>,
    /// Request header whose value will be excluded from inspection during preconfigured WAF evaluation.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestHeader>>,
    /// Request query parameter whose value will be excluded from inspection during preconfigured WAF evaluation.  Note that the parameter can be in the query string or in the POST body.
    #[builder(into)]
    #[serde(rename = "requestQueryParams")]
    pub r#request_query_params: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam>>,
    /// Request URI from the request line to be excluded from inspection during preconfigured WAF evaluation. When specifying this field, the query or fragment part should be excluded.
    #[builder(into)]
    #[serde(rename = "requestUris")]
    pub r#request_uris: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri>>,
    /// A list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion. If omitted, it refers to all the rule IDs under the WAF rule set.
    #[builder(into)]
    #[serde(rename = "targetRuleIds")]
    pub r#target_rule_ids: Box<Vec<String>>,
    /// Target WAF rule set to apply the preconfigured WAF exclusion.
    #[builder(into)]
    #[serde(rename = "targetRuleSet")]
    pub r#target_rule_set: Box<String>,
}
