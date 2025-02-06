#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilterTablesIncludeRegexesPattern {
    /// if unset, this property matches all datasets
    #[builder(into, default)]
    #[serde(rename = "datasetIdRegex")]
    pub r#dataset_id_regex: Box<Option<String>>,
    /// For organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project.
    #[builder(into, default)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Box<Option<String>>,
    /// if unset, this property matches all tables
    #[builder(into, default)]
    #[serde(rename = "tableIdRegex")]
    pub r#table_id_regex: Box<Option<String>>,
}
