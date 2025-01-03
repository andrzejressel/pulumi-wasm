#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchemaPostgresqlTable {
    /// PostgreSQL columns in the schema. When unspecified as part of include/exclude objects, includes/excludes everything.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postgresqlColumns")]
    pub r#postgresql_columns: Box<Option<Vec<super::super::types::datastream::StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn>>>,
    /// Table name.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<String>,
}
