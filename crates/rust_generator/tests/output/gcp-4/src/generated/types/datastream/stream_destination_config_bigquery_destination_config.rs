#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamDestinationConfigBigqueryDestinationConfig {
    /// AppendOnly mode defines that the stream of changes (INSERT, UPDATE-INSERT, UPDATE-DELETE and DELETE
    /// events) to a source table will be written to the destination Google BigQuery table, retaining the
    /// historical state of the data.
    #[builder(into, default)]
    #[serde(rename = "appendOnly")]
    pub r#append_only: Box<Option<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigAppendOnly>>,
    /// The guaranteed data freshness (in seconds) when querying tables created by the stream.
    /// Editing this field will only affect new tables created in the future, but existing tables
    /// will not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". Defaults to 900s.
    #[builder(into, default)]
    #[serde(rename = "dataFreshness")]
    pub r#data_freshness: Box<Option<String>>,
    /// Merge mode defines that all changes to a table will be merged at the destination Google BigQuery
    /// table. This is the default write mode. When selected, BigQuery reflects the way the data is stored
    /// in the source database. With Merge mode, no historical record of the change events is kept.
    #[builder(into, default)]
    #[serde(rename = "merge")]
    pub r#merge: Box<Option<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigMerge>>,
    /// A single target dataset to which all data will be streamed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "singleTargetDataset")]
    pub r#single_target_dataset: Box<Option<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigSingleTargetDataset>>,
    /// Destination datasets are created so that hierarchy of the destination data objects matches the source hierarchy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceHierarchyDatasets")]
    pub r#source_hierarchy_datasets: Box<Option<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets>>,
}
