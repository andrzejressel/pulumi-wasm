#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatement {
    /// Logical rule statement used to combine other rule statements with AND logic. See `and_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "andStatement")]
    pub r#and_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementAndStatement>>,
    /// Rule statement that defines a string match search for AWS WAF to apply to web requests. See `byte_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "byteMatchStatement")]
    pub r#byte_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementByteMatchStatement>>,
    /// Rule statement used to identify web requests based on country of origin. See `geo_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "geoMatchStatement")]
    pub r#geo_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementGeoMatchStatement>>,
    /// Rule statement used to detect web requests coming from particular IP addresses or address ranges. See `ip_set_reference_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "ipSetReferenceStatement")]
    pub r#ip_set_reference_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementIpSetReferenceStatement>>,
    /// Rule statement that defines a string match search against labels that have been added to the web request by rules that have already run in the web ACL. See `label_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "labelMatchStatement")]
    pub r#label_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementLabelMatchStatement>>,
    /// Rule statement used to run the rules that are defined in a managed rule group.  This statement can not be nested. See `managed_rule_group_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "managedRuleGroupStatement")]
    pub r#managed_rule_group_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatement>>,
    /// Logical rule statement used to negate the results of another rule statement. See `not_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "notStatement")]
    pub r#not_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementNotStatement>>,
    /// Logical rule statement used to combine other rule statements with OR logic. See `or_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "orStatement")]
    pub r#or_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementOrStatement>>,
    /// Rate-based rule tracks the rate of requests for each originating `IP address`, and triggers the rule action when the rate exceeds a limit that you specify on the number of requests in any `5-minute` time span. This statement can not be nested. See `rate_based_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "rateBasedStatement")]
    pub r#rate_based_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementRateBasedStatement>>,
    /// Rule statement used to search web request components for a match against a single regular expression. See `regex_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "regexMatchStatement")]
    pub r#regex_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementRegexMatchStatement>>,
    /// Rule statement used to search web request components for matches with regular expressions. See `regex_pattern_set_reference_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "regexPatternSetReferenceStatement")]
    pub r#regex_pattern_set_reference_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementRegexPatternSetReferenceStatement>>,
    /// Rule statement used to run the rules that are defined in an WAFv2 Rule Group. See `rule_group_reference_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "ruleGroupReferenceStatement")]
    pub r#rule_group_reference_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatement>>,
    /// Rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). See `size_constraint_statement` below for more details.
    #[builder(into, default)]
    #[serde(rename = "sizeConstraintStatement")]
    pub r#size_constraint_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatement>>,
    /// An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. See `sqli_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "sqliMatchStatement")]
    pub r#sqli_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatement>>,
    /// Rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. See `xss_match_statement` below for details.
    #[builder(into, default)]
    #[serde(rename = "xssMatchStatement")]
    pub r#xss_match_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementXssMatchStatement>>,
}
