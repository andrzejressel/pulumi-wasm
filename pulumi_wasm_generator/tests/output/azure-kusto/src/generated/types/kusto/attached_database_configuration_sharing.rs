#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttachedDatabaseConfigurationSharing {
    /// List of external tables exclude from the follower database.
    #[builder(into, default)]
    #[serde(rename = "externalTablesToExcludes")]
    pub r#external_tables_to_excludes: Box<Option<Vec<String>>>,
    /// List of external tables to include in the follower database.
    #[builder(into, default)]
    #[serde(rename = "externalTablesToIncludes")]
    pub r#external_tables_to_includes: Box<Option<Vec<String>>>,
    /// List of materialized views exclude from the follower database.
    #[builder(into, default)]
    #[serde(rename = "materializedViewsToExcludes")]
    pub r#materialized_views_to_excludes: Box<Option<Vec<String>>>,
    /// List of materialized views to include in the follower database.
    #[builder(into, default)]
    #[serde(rename = "materializedViewsToIncludes")]
    pub r#materialized_views_to_includes: Box<Option<Vec<String>>>,
    /// List of tables to exclude from the follower database.
    #[builder(into, default)]
    #[serde(rename = "tablesToExcludes")]
    pub r#tables_to_excludes: Box<Option<Vec<String>>>,
    /// List of tables to include in the follower database.
    #[builder(into, default)]
    #[serde(rename = "tablesToIncludes")]
    pub r#tables_to_includes: Box<Option<Vec<String>>>,
}
