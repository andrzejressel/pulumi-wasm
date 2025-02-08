#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapSource {
    /// ARN of the parent data set.
    #[builder(into, default)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: Box<Option<String>>,
    /// Specifies the result of a join of two logical tables. See join_instruction.
    #[builder(into, default)]
    #[serde(rename = "joinInstruction")]
    pub r#join_instruction: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstruction>>,
    /// Physical table ID.
    #[builder(into, default)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: Box<Option<String>>,
}
