#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisAlternatePathHint {
    /// The Amazon Resource Name (ARN) of the component.
    #[builder(into, default)]
    #[serde(rename = "componentArn")]
    pub r#component_arn: Box<Option<String>>,
    /// The ID of the component.
    #[builder(into, default)]
    #[serde(rename = "componentId")]
    pub r#component_id: Box<Option<String>>,
}
