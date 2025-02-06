#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineJobReconciliationPipelineJob {
    /// The harmonized FHIR store to write harmonized FHIR resources to,
    /// in the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}
    #[builder(into, default)]
    #[serde(rename = "fhirStoreDestination")]
    pub r#fhir_store_destination: Box<Option<String>>,
    /// Specifies the top level directory of the matching configs used
    /// in all mapping pipelines, which extract properties for resources
    /// to be matched on.
    /// Example: gs://{bucket-id}/{path/to/matching/configs}
    #[builder(into)]
    #[serde(rename = "matchingUriPrefix")]
    pub r#matching_uri_prefix: Box<String>,
    /// Specifies the location of the reconciliation configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mergeConfig")]
    pub r#merge_config: Box<super::super::types::healthcare::PipelineJobReconciliationPipelineJobMergeConfig>,
}
