#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
