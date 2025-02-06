#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetPhysicalTableMapCustomSql {
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapCustomSqlColumn>>,
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "sqlQuery")]
    pub r#sql_query: Box<String>,
}
