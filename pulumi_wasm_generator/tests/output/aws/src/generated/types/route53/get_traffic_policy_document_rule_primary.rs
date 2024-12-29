#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTrafficPolicyDocumentRulePrimary {
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
    /// References to a rule.
    #[builder(into, default)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Box<Option<String>>,
}
