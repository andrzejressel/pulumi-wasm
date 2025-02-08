#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureGroupOfflineStoreConfigDataCatalogConfig {
    /// The name of the Glue table catalog.
    #[builder(into, default)]
    #[serde(rename = "catalog")]
    pub r#catalog: Box<Option<String>>,
    /// The name of the Glue table database.
    #[builder(into, default)]
    #[serde(rename = "database")]
    pub r#database: Box<Option<String>>,
    /// The name of the Glue table.
    #[builder(into, default)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<Option<String>>,
}
