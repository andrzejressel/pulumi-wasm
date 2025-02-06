#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetPhysicalTableMapCustomSql {
    /// Column schema from the SQL query result set. See columns.
    #[builder(into, default)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Option<Vec<super::super::types::quicksight::DataSetPhysicalTableMapCustomSqlColumn>>>,
    /// ARN of the data source.
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    /// Display name for the SQL query result.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// SQL query.
    #[builder(into)]
    #[serde(rename = "sqlQuery")]
    pub r#sql_query: Box<String>,
}
