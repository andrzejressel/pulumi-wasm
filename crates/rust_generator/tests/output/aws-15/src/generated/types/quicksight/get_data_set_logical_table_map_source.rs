#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataSetLogicalTableMapSource {
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "joinInstructions")]
    pub r#join_instructions: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstruction>>,
    #[builder(into)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: Box<String>,
}
