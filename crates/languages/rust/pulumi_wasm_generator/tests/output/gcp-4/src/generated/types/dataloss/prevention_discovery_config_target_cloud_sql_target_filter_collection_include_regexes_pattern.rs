#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollectionIncludeRegexesPattern {
    /// Regex to test the database name against. If empty, all databases match.
    #[builder(into, default)]
    #[serde(rename = "databaseRegex")]
    pub r#database_regex: Box<Option<String>>,
    /// Regex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'
    #[builder(into, default)]
    #[serde(rename = "databaseResourceNameRegex")]
    pub r#database_resource_name_regex: Box<Option<String>>,
    /// Regex to test the instance name against. If empty, all instances match.
    #[builder(into, default)]
    #[serde(rename = "instanceRegex")]
    pub r#instance_regex: Box<Option<String>>,
    /// For organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project.
    #[builder(into, default)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Box<Option<String>>,
}
