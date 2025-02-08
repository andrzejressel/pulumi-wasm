#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CrawlerDeltaTarget {
    /// The name of the connection to use to connect to the Delta table target.
    #[builder(into, default)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<Option<String>>,
    /// Specifies whether the crawler will create native tables, to allow integration with query engines that support querying of the Delta transaction log directly.
    #[builder(into, default)]
    #[serde(rename = "createNativeDeltaTable")]
    pub r#create_native_delta_table: Box<Option<bool>>,
    /// A list of the Amazon S3 paths to the Delta tables.
    #[builder(into)]
    #[serde(rename = "deltaTables")]
    pub r#delta_tables: Box<Vec<String>>,
    /// Specifies whether to write the manifest files to the Delta table path.
    #[builder(into)]
    #[serde(rename = "writeManifest")]
    pub r#write_manifest: Box<bool>,
}
