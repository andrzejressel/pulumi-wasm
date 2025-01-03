#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableTableConstraintsForeignKeyColumnReferences {
    /// The column in the primary key that are
    /// referenced by the referencingColumn
    #[builder(into)]
    #[serde(rename = "referencedColumn")]
    pub r#referenced_column: Box<String>,
    /// The column that composes the foreign key.
    #[builder(into)]
    #[serde(rename = "referencingColumn")]
    pub r#referencing_column: Box<String>,
}
