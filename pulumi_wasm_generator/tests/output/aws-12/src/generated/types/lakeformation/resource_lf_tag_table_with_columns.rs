#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceLfTagTableWithColumns {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into, default)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<Option<String>>,
    /// Set of column names for the table.
    #[builder(into, default)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Box<Option<Vec<String>>>,
    /// Option to add column wildcard. See Column Wildcard for more details.
    #[builder(into, default)]
    #[serde(rename = "columnWildcard")]
    pub r#column_wildcard: Box<Option<super::super::types::lakeformation::ResourceLfTagTableWithColumnsColumnWildcard>>,
    /// Name of the database for the table with columns resource. Unique to the Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Name of the table resource.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
