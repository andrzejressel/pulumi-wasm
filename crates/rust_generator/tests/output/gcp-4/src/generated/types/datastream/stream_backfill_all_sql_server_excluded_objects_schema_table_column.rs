#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamBackfillAllSqlServerExcludedObjectsSchemaTableColumn {
    /// Column name.
    #[builder(into, default)]
    #[serde(rename = "column")]
    pub r#column: Box<Option<String>>,
    /// The SQL Server data type. Full data types list can be found here:
    /// https://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16
    #[builder(into, default)]
    #[serde(rename = "dataType")]
    pub r#data_type: Box<Option<String>>,
    /// (Output)
    /// Column length.
    #[builder(into, default)]
    #[serde(rename = "length")]
    pub r#length: Box<Option<i32>>,
    /// (Output)
    /// Whether or not the column can accept a null value.
    #[builder(into, default)]
    #[serde(rename = "nullable")]
    pub r#nullable: Box<Option<bool>>,
    /// (Output)
    /// The ordinal position of the column in the table.
    #[builder(into, default)]
    #[serde(rename = "ordinalPosition")]
    pub r#ordinal_position: Box<Option<i32>>,
    /// (Output)
    /// Column precision.
    #[builder(into, default)]
    #[serde(rename = "precision")]
    pub r#precision: Box<Option<i32>>,
    /// (Output)
    /// Whether or not the column represents a primary key.
    #[builder(into, default)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<Option<bool>>,
    /// (Output)
    /// Column scale.
    #[builder(into, default)]
    #[serde(rename = "scale")]
    pub r#scale: Box<Option<i32>>,
}
