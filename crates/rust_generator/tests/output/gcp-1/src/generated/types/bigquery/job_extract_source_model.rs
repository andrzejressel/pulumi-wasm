#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobExtractSourceModel {
    /// The ID of the dataset containing this model.
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<String>,
    /// The ID of the model.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "modelId")]
    pub r#model_id: Box<String>,
    /// The ID of the project containing this model.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
