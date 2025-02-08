#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetDiscoveryStatusStat {
    /// The count of data items within the referenced resource.
    #[builder(into, default)]
    #[serde(rename = "dataItems")]
    pub r#data_items: Box<Option<i32>>,
    /// The number of stored data bytes within the referenced resource.
    #[builder(into, default)]
    #[serde(rename = "dataSize")]
    pub r#data_size: Box<Option<i32>>,
    /// The count of fileset entities within the referenced resource.
    #[builder(into, default)]
    #[serde(rename = "filesets")]
    pub r#filesets: Box<Option<i32>>,
    /// The count of table entities within the referenced resource.
    #[builder(into, default)]
    #[serde(rename = "tables")]
    pub r#tables: Box<Option<i32>>,
}
