#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TemplateSourceEntity {
    /// The source analysis, if it is based on an analysis.. Only one of `source_analysis` or `source_template` should be configured. See source_analysis.
    #[builder(into, default)]
    #[serde(rename = "sourceAnalysis")]
    pub r#source_analysis: Box<Option<super::super::types::quicksight::TemplateSourceEntitySourceAnalysis>>,
    /// The source template, if it is based on an template.. Only one of `source_analysis` or `source_template` should be configured. See source_template.
    #[builder(into, default)]
    #[serde(rename = "sourceTemplate")]
    pub r#source_template: Box<Option<super::super::types::quicksight::TemplateSourceEntitySourceTemplate>>,
}
