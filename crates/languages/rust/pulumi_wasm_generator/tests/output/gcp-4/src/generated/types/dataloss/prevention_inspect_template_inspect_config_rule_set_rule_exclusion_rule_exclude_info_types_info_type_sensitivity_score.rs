#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypesInfoTypeSensitivityScore {
    /// The sensitivity score applied to the resource.
    /// Possible values are: `SENSITIVITY_LOW`, `SENSITIVITY_MODERATE`, `SENSITIVITY_HIGH`.
    #[builder(into)]
    #[serde(rename = "score")]
    pub r#score: Box<String>,
}
