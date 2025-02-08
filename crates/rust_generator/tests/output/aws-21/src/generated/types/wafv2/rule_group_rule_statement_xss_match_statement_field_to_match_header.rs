#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupRuleStatementXssMatchStatementFieldToMatchHeader {
    /// The filter to use to identify the subset of headers to inspect in a web request. The `match_pattern` block supports only one of the following arguments:
    #[builder(into)]
    #[serde(rename = "matchPattern")]
    pub r#match_pattern: Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchHeaderMatchPattern>,
    /// The parts of the headers to inspect with the rule inspection criteria. If you specify `All`, AWS WAF inspects both keys and values. Valid values include the following: `ALL`, `Key`, `Value`.
    #[builder(into)]
    #[serde(rename = "matchScope")]
    pub r#match_scope: Box<String>,
    /// Oversize handling tells AWS WAF what to do with a web request when the request component that the rule inspects is over the limits. Valid values include the following: `CONTINUE`, `MATCH`, `NO_MATCH`. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statement-oversize-handling.html) for more information.
    #[builder(into)]
    #[serde(rename = "oversizeHandling")]
    pub r#oversize_handling: Box<String>,
}
