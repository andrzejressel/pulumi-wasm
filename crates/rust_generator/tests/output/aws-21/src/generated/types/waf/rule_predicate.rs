#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RulePredicate {
    /// A unique identifier for a predicate in the rule, such as Byte Match Set ID or IPSet ID.
    #[builder(into)]
    #[serde(rename = "dataId")]
    pub r#data_id: Box<String>,
    /// Set this to `false` if you want to allow, block, or count requests
    /// based on the settings in the specified waf_byte_match_set, waf_ipset, aws_waf_size_constraint_set, aws.waf.SqlInjectionMatchSet or aws_waf_xss_match_set.
    /// For example, if an IPSet includes the IP address `192.0.2.44`, AWS WAF will allow or block requests based on that IP address.
    /// If set to `true`, AWS WAF will allow, block, or count requests based on all IP addresses except `192.0.2.44`.
    #[builder(into)]
    #[serde(rename = "negated")]
    pub r#negated: Box<bool>,
    /// The type of predicate in a rule. Valid values: `ByteMatch`, `GeoMatch`, `IPMatch`, `RegexMatch`, `SizeConstraint`, `SqlInjectionMatch`, or `XssMatch`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
