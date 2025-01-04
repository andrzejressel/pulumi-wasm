#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityRulesetTargetTable {
    /// The catalog id where the AWS Glue table exists.
    #[builder(into, default)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<Option<String>>,
    /// Name of the database where the AWS Glue table exists.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Name of the AWS Glue table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
}
