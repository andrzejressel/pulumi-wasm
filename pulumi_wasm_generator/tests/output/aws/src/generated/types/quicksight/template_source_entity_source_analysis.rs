#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TemplateSourceEntitySourceAnalysis {
    /// The Amazon Resource Name (ARN) of the resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// A list of dataset references used as placeholders in the template. See data_set_references.
    #[builder(into)]
    #[serde(rename = "dataSetReferences")]
    pub r#data_set_references: Box<Vec<super::super::types::quicksight::TemplateSourceEntitySourceAnalysisDataSetReference>>,
}
