#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatasetExternalDatasetReference {
    /// The connection id that is used to access the externalSource.
    /// Format: projects/{projectId}/locations/{locationId}/connections/{connectionId}
    #[builder(into)]
    #[serde(rename = "connection")]
    pub r#connection: Box<String>,
    /// External source that backs this dataset.
    #[builder(into)]
    #[serde(rename = "externalSource")]
    pub r#external_source: Box<String>,
}
