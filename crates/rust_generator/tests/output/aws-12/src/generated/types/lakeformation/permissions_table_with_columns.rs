#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PermissionsTableWithColumns {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into, default)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<Option<String>>,
    /// Set of column names for the table.
    #[builder(into, default)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Box<Option<Vec<String>>>,
    /// Name of the database for the table with columns resource. Unique to the Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Set of column names for the table to exclude. If `excluded_column_names` is included, `wildcard` must be set to `true` to avoid the provider reporting a difference.
    #[builder(into, default)]
    #[serde(rename = "excludedColumnNames")]
    pub r#excluded_column_names: Box<Option<Vec<String>>>,
    /// Name of the table resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Whether to use a column wildcard. If `excluded_column_names` is included, `wildcard` must be set to `true` to avoid the provider reporting a difference.
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}
