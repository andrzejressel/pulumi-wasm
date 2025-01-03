#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetConditions {
    /// Database engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified.
    /// Each value may be one of: `ALL_SUPPORTED_DATABASE_ENGINES`, `MYSQL`, `POSTGRES`.
    #[builder(into, default)]
    #[serde(rename = "databaseEngines")]
    pub r#database_engines: Box<Option<Vec<String>>>,
    /// Data profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES].
    /// Each value may be one of: `DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES`, `DATABASE_RESOURCE_TYPE_TABLE`.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<String>>>,
}
