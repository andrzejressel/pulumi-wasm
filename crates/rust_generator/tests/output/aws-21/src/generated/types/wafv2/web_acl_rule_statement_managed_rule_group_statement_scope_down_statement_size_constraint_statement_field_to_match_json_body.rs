#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementSizeConstraintStatementFieldToMatchJsonBody {
    /// What to do when JSON parsing fails. Defaults to evaluating up to the first parsing failure. Valid values are `EVALUATE_AS_STRING`, `MATCH` and `NO_MATCH`.
    #[builder(into, default)]
    #[serde(rename = "invalidFallbackBehavior")]
    pub r#invalid_fallback_behavior: Box<Option<String>>,
    /// The patterns to look for in the JSON body. You must specify exactly one setting: either `all` or `included_paths`. See [JsonMatchPattern](https://docs.aws.amazon.com/waf/latest/APIReference/API_JsonMatchPattern.html) for details.
    #[builder(into)]
    #[serde(rename = "matchPattern")]
    pub r#match_pattern: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementSizeConstraintStatementFieldToMatchJsonBodyMatchPattern>,
    /// The parts of the JSON to match against using the `match_pattern`. Valid values are `ALL`, `KEY` and `VALUE`.
    #[builder(into)]
    #[serde(rename = "matchScope")]
    pub r#match_scope: Box<String>,
    /// What to do if the body is larger than can be inspected. Valid values are `CONTINUE` (default), `MATCH` and `NO_MATCH`.
    #[builder(into, default)]
    #[serde(rename = "oversizeHandling")]
    pub r#oversize_handling: Box<Option<String>>,
}
