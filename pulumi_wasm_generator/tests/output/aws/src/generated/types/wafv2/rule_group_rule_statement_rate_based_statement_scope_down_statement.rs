#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatement {
    /// A logical rule statement used to combine other rule statements with AND logic. See AND Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "andStatement")]
    pub r#and_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementAndStatement>>,
    /// A rule statement that defines a string match search for AWS WAF to apply to web requests. See Byte Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "byteMatchStatement")]
    pub r#byte_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatement>>,
    /// A rule statement used to identify web requests based on country of origin. See GEO Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "geoMatchStatement")]
    pub r#geo_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementGeoMatchStatement>>,
    /// A rule statement used to detect web requests coming from particular IP addresses or address ranges. See IP Set Reference Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "ipSetReferenceStatement")]
    pub r#ip_set_reference_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementIpSetReferenceStatement>>,
    /// A rule statement that defines a string match search against labels that have been added to the web request by rules that have already run in the web ACL. See Label Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "labelMatchStatement")]
    pub r#label_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementLabelMatchStatement>>,
    /// A logical rule statement used to negate the results of another rule statement. See NOT Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "notStatement")]
    pub r#not_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementNotStatement>>,
    /// A logical rule statement used to combine other rule statements with OR logic. See OR Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "orStatement")]
    pub r#or_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementOrStatement>>,
    /// A rule statement used to search web request components for a match against a single regular expression. See Regex Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "regexMatchStatement")]
    pub r#regex_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexMatchStatement>>,
    /// A rule statement used to search web request components for matches with regular expressions. See Regex Pattern Set Reference Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "regexPatternSetReferenceStatement")]
    pub r#regex_pattern_set_reference_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementRegexPatternSetReferenceStatement>>,
    /// A rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). See Size Constraint Statement below for more details.
    #[builder(into, default)]
    #[serde(rename = "sizeConstraintStatement")]
    pub r#size_constraint_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatement>>,
    /// An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. See SQL Injection Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "sqliMatchStatement")]
    pub r#sqli_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatement>>,
    /// A rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. See XSS Match Statement below for details.
    #[builder(into, default)]
    #[serde(rename = "xssMatchStatement")]
    pub r#xss_match_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementXssMatchStatement>>,
}