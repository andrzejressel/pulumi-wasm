#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamSourceConfigOracleSourceConfigIncludeObjects {
    /// Oracle schemas/databases in the database server
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleSchemas")]
    pub r#oracle_schemas: Box<Vec<super::super::types::datastream::StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchema>>,
}
