#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyRulePreconfiguredWafConfigExclusion {
    /// Request cookie whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestCookies")]
    pub r#request_cookies: Box<Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestCooky>>>,
    /// Request header whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Box<Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestHeader>>>,
    /// Request query parameter whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Note that the parameter can be in the query string or in the POST body.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestQueryParams")]
    pub r#request_query_params: Box<Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam>>>,
    /// Request URI from the request line to be excluded from inspection during preconfigured WAF evaluation.
    /// When specifying this field, the query or fragment part should be excluded.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestUris")]
    pub r#request_uris: Box<Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri>>>,
    /// A list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.
    /// If omitted, it refers to all the rule IDs under the WAF rule set.
    #[builder(into, default)]
    #[serde(rename = "targetRuleIds")]
    pub r#target_rule_ids: Box<Option<Vec<String>>>,
    /// Target WAF rule set to apply the preconfigured WAF exclusion.
    #[builder(into)]
    #[serde(rename = "targetRuleSet")]
    pub r#target_rule_set: Box<String>,
}
