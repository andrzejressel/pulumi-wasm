#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogDatabaseTargetDatabase {
    /// ID of the Data Catalog in which the database resides.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<String>,
    /// Name of the catalog database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Region of the target database.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}