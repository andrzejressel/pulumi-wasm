#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCellsFilterTableData {
    /// A list of column names and/or nested column attributes.
    #[builder(into, default)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Box<Option<Vec<String>>>,
    /// A wildcard with exclusions. See Column Wildcard below for details.
    #[builder(into, default)]
    #[serde(rename = "columnWildcard")]
    pub r#column_wildcard: Box<Option<super::super::types::lakeformation::DataCellsFilterTableDataColumnWildcard>>,
    /// The name of the database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// The name of the data cells filter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A PartiQL predicate. See Row Filter below for details.
    #[builder(into, default)]
    #[serde(rename = "rowFilter")]
    pub r#row_filter: Box<Option<super::super::types::lakeformation::DataCellsFilterTableDataRowFilter>>,
    /// The ID of the Data Catalog.
    #[builder(into)]
    #[serde(rename = "tableCatalogId")]
    pub r#table_catalog_id: Box<String>,
    /// The name of the table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
    /// ID of the data cells filter version.
    #[builder(into, default)]
    #[serde(rename = "versionId")]
    pub r#version_id: Box<Option<String>>,
}
