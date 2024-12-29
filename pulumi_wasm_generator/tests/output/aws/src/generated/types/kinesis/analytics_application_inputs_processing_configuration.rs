#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationInputsProcessingConfiguration {
    /// The Lambda function configuration. See Lambda below for more details.
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfigurationLambda>,
}
