#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfiguration {
    /// Disables real time alert rules.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Collection of real time alert rules
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule>>,
}