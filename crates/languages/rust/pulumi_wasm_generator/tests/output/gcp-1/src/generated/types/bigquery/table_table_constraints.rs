#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableTableConstraints {
    /// Present only if the table has a foreign key.
    /// The foreign key is not enforced.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "foreignKeys")]
    pub r#foreign_keys: Box<Option<Vec<super::super::types::bigquery::TableTableConstraintsForeignKey>>>,
    /// Represents the primary key constraint
    /// on a table's columns. Present only if the table has a primary key.
    /// The primary key is not enforced.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<Option<super::super::types::bigquery::TableTableConstraintsPrimaryKey>>,
}
