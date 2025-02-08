#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobStorageConfig {
    /// Options defining BigQuery table and row identifiers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigQueryOptions")]
    pub r#big_query_options: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptions>>,
    /// Options defining a file or a set of files within a Google Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStorageOptions")]
    pub r#cloud_storage_options: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions>>,
    /// Options defining a data set within Google Cloud Datastore.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "datastoreOptions")]
    pub r#datastore_options: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigDatastoreOptions>>,
    /// Configuration to control jobs where the content being inspected is outside of Google Cloud Platform.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hybridOptions")]
    pub r#hybrid_options: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigHybridOptions>>,
    /// Configuration of the timespan of the items to include in scanning
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timespanConfig")]
    pub r#timespan_config: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigTimespanConfig>>,
}
