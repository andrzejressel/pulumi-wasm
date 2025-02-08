#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobCopySourceTable {
    /// The ID of the dataset containing this table.
    #[builder(into, default)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<Option<String>>,
    /// The ID of the project containing this table.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
    /// The table. Can be specified `{{table_id}}` if `project_id` and `dataset_id` are also set,
    /// or of the form `projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}` if not.
    #[builder(into)]
    #[serde(rename = "tableId")]
    pub r#table_id: Box<String>,
}
