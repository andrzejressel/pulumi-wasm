#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisAlternatePathHint {
    #[builder(into)]
    #[serde(rename = "componentArn")]
    pub r#component_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: Box<String>,
}