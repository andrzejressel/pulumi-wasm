#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchema {
    /// Schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
    /// Tables in the database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tables")]
    pub r#tables: Box<Option<Vec<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTable>>>,
}
