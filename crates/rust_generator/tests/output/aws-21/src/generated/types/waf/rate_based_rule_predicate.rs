#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RateBasedRulePredicate {
    /// A unique identifier for a predicate in the rule, such as Byte Match Set ID or IPSet ID.
    #[builder(into)]
    #[serde(rename = "dataId")]
    pub r#data_id: Box<String>,
    /// Set this to `false` if you want to allow, block, or count requests
    /// based on the settings in the specified `ByteMatchSet`, `IPSet`, `SqlInjectionMatchSet`, `XssMatchSet`, or `SizeConstraintSet`.
    /// For example, if an IPSet includes the IP address `192.0.2.44`, AWS WAF will allow or block requests based on that IP address.
    /// If set to `true`, AWS WAF will allow, block, or count requests based on all IP addresses _except_ `192.0.2.44`.
    #[builder(into)]
    #[serde(rename = "negated")]
    pub r#negated: Box<bool>,
    /// The type of predicate in a rule. Valid values: `ByteMatch`, `GeoMatch`, `IPMatch`, `RegexMatch`, `SizeConstraint`, `SqlInjectionMatch`, or `XssMatch`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
