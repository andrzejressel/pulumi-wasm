#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AnalysisSourceEntitySourceTemplate {
    /// The Amazon Resource Name (ARN) of the resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// List of dataset references. See data_set_references.
    #[builder(into)]
    #[serde(rename = "dataSetReferences")]
    pub r#data_set_references: Box<Vec<super::super::types::quicksight::AnalysisSourceEntitySourceTemplateDataSetReference>>,
}
