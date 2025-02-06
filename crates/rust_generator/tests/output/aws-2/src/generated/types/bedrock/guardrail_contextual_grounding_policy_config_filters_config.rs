#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailContextualGroundingPolicyConfigFiltersConfig {
    /// The threshold for this filter.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
    /// Type of contextual grounding filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
