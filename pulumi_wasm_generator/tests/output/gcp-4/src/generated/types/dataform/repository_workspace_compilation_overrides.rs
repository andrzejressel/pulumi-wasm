#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryWorkspaceCompilationOverrides {
    /// The default database (Google Cloud project ID).
    #[builder(into, default)]
    #[serde(rename = "defaultDatabase")]
    pub r#default_database: Box<Option<String>>,
    /// The suffix that should be appended to all schema (BigQuery dataset ID) names.
    #[builder(into, default)]
    #[serde(rename = "schemaSuffix")]
    pub r#schema_suffix: Box<Option<String>>,
    /// The prefix that should be prepended to all table names.
    #[builder(into, default)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Box<Option<String>>,
}
