#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetDataSetUsageConfiguration {
    /// Controls whether a child dataset of a direct query can use this dataset as a source.
    #[builder(into, default)]
    #[serde(rename = "disableUseAsDirectQuerySource")]
    pub r#disable_use_as_direct_query_source: Box<Option<bool>>,
    /// Controls whether a child dataset that's stored in QuickSight can use this dataset as a source.
    #[builder(into, default)]
    #[serde(rename = "disableUseAsImportedSource")]
    pub r#disable_use_as_imported_source: Box<Option<bool>>,
}
