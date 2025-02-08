#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetPhysicalTableMapRelationalTable {
    /// Catalog associated with the table.
    #[builder(into, default)]
    #[serde(rename = "catalog")]
    pub r#catalog: Box<Option<String>>,
    /// ARN of the data source.
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    /// Column schema of the table. See input_columns.
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Box<Vec<super::super::types::quicksight::DataSetPhysicalTableMapRelationalTableInputColumn>>,
    /// Name of the relational table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Schema name. This name applies to certain relational database engines.
    #[builder(into, default)]
    #[serde(rename = "schema")]
    pub r#schema: Box<Option<String>>,
}
