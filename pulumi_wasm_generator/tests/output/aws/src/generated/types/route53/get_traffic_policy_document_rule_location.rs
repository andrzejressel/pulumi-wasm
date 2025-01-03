#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTrafficPolicyDocumentRuleLocation {
    /// Value of a continent.
    #[builder(into, default)]
    #[serde(rename = "continent")]
    pub r#continent: Box<Option<String>>,
    /// Value of a country.
    #[builder(into, default)]
    #[serde(rename = "country")]
    pub r#country: Box<Option<String>>,
    /// References to an endpoint.
    #[builder(into, default)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Box<Option<String>>,
    /// Indicates whether you want Amazon Route 53 to evaluate the health of the endpoint and route traffic only to healthy endpoints.
    #[builder(into, default)]
    #[serde(rename = "evaluateTargetHealth")]
    pub r#evaluate_target_health: Box<Option<bool>>,
    /// If you want to associate a health check with the endpoint or rule.
    #[builder(into, default)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<Option<String>>,
    /// Indicates whether this set of values represents the default location.
    #[builder(into, default)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Box<Option<bool>>,
    /// References to a rule.
    #[builder(into, default)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Box<Option<String>>,
    /// Value of a subdivision.
    #[builder(into, default)]
    #[serde(rename = "subdivision")]
    pub r#subdivision: Box<Option<String>>,
}
