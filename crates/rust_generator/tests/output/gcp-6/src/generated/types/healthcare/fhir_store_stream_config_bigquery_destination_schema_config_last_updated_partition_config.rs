#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FhirStoreStreamConfigBigqueryDestinationSchemaConfigLastUpdatedPartitionConfig {
    /// Number of milliseconds for which to keep the storage for a partition.
    #[builder(into, default)]
    #[serde(rename = "expirationMs")]
    pub r#expiration_ms: Box<Option<String>>,
    /// Type of partitioning.
    /// Possible values are: `PARTITION_TYPE_UNSPECIFIED`, `HOUR`, `DAY`, `MONTH`, `YEAR`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
