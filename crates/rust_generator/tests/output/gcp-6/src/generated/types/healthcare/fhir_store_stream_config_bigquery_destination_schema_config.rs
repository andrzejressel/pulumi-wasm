#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FhirStoreStreamConfigBigqueryDestinationSchemaConfig {
    /// The configuration for exported BigQuery tables to be partitioned by FHIR resource's last updated time column.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "lastUpdatedPartitionConfig")]
    pub r#last_updated_partition_config: Box<Option<super::super::types::healthcare::FhirStoreStreamConfigBigqueryDestinationSchemaConfigLastUpdatedPartitionConfig>>,
    /// The depth for all recursive structures in the output analytics schema. For example, concept in the CodeSystem
    /// resource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called
    /// concept.concept but not concept.concept.concept. If not specified or set to 0, the server will use the default
    /// value 2. The maximum depth allowed is 5.
    #[builder(into)]
    #[serde(rename = "recursiveStructureDepth")]
    pub r#recursive_structure_depth: Box<i32>,
    /// Specifies the output schema type.
    /// * ANALYTICS: Analytics schema defined by the FHIR community.
    /// See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md.
    /// * ANALYTICS_V2: Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON.
    /// * LOSSLESS: A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification.
    /// Default value is `ANALYTICS`.
    /// Possible values are: `ANALYTICS`, `ANALYTICS_V2`, `LOSSLESS`.
    #[builder(into, default)]
    #[serde(rename = "schemaType")]
    pub r#schema_type: Box<Option<String>>,
}
