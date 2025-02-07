#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetPhysicalTableMapRelationalTable {
    #[builder(into)]
    #[serde(rename = "catalog")]
    pub r#catalog: Box<String>,
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Box<Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapRelationalTableInputColumn>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
}
