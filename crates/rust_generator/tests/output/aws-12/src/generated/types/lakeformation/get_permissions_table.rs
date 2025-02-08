#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPermissionsTable {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<String>,
    /// Name of the database for the table. Unique to a Data Catalog.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Name of the table. At least one of `name` or `wildcard` is required.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Whether to use a wildcard representing every table under a database. At least one of `name` or `wildcard` is required. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}
