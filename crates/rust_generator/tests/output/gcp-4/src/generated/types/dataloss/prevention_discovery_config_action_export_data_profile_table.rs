#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigActionExportDataProfileTable {
    /// Dataset Id of the table
    #[builder(into, default)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<Option<String>>,
    /// The Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
    /// Name of the table
    #[builder(into, default)]
    #[serde(rename = "tableId")]
    pub r#table_id: Box<Option<String>>,
}
