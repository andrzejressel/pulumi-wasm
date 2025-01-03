#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TemplateSourceEntitySourceAnalysisDataSetReference {
    /// Dataset Amazon Resource Name (ARN).
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: Box<String>,
    /// Dataset placeholder.
    #[builder(into)]
    #[serde(rename = "dataSetPlaceholder")]
    pub r#data_set_placeholder: Box<String>,
}
