#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FhirStoreStreamConfigBigqueryDestination {
    /// BigQuery URI to a dataset, up to 2000 characters long, in the format bq://projectId.bqDatasetId
    #[builder(into)]
    #[serde(rename = "datasetUri")]
    pub r#dataset_uri: Box<String>,
    /// The configuration for the exported BigQuery schema.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schemaConfig")]
    pub r#schema_config: Box<super::super::types::healthcare::FhirStoreStreamConfigBigqueryDestinationSchemaConfig>,
}
