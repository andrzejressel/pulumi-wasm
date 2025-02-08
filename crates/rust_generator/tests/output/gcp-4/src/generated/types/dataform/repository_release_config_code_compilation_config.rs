#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RepositoryReleaseConfigCodeCompilationConfig {
    /// Optional. The default schema (BigQuery dataset ID) for assertions.
    #[builder(into, default)]
    #[serde(rename = "assertionSchema")]
    pub r#assertion_schema: Box<Option<String>>,
    /// Optional. The suffix that should be appended to all database (Google Cloud project ID) names.
    #[builder(into, default)]
    #[serde(rename = "databaseSuffix")]
    pub r#database_suffix: Box<Option<String>>,
    /// Optional. The default database (Google Cloud project ID).
    #[builder(into, default)]
    #[serde(rename = "defaultDatabase")]
    pub r#default_database: Box<Option<String>>,
    /// Optional. The default BigQuery location to use. Defaults to "US".
    /// See the BigQuery docs for a full list of locations: https://cloud.google.com/bigquery/docs/locations.
    #[builder(into, default)]
    #[serde(rename = "defaultLocation")]
    pub r#default_location: Box<Option<String>>,
    /// Optional. The default schema (BigQuery dataset ID).
    #[builder(into, default)]
    #[serde(rename = "defaultSchema")]
    pub r#default_schema: Box<Option<String>>,
    /// Optional. The suffix that should be appended to all schema (BigQuery dataset ID) names.
    #[builder(into, default)]
    #[serde(rename = "schemaSuffix")]
    pub r#schema_suffix: Box<Option<String>>,
    /// Optional. The prefix that should be prepended to all table names.
    #[builder(into, default)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Box<Option<String>>,
    /// Optional. User-defined variables that are made available to project code during compilation.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into, default)]
    #[serde(rename = "vars")]
    pub r#vars: Box<Option<std::collections::HashMap<String, String>>>,
}
