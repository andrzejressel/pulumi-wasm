#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration {
    /// SNS topic to deliver results.
    #[builder(into)]
    #[serde(rename = "insightsTarget")]
    pub r#insights_target: Box<String>,
}
