#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetLogicalTableMapSourceJoinInstruction {
    #[builder(into)]
    #[serde(rename = "leftJoinKeyProperties")]
    pub r#left_join_key_properties: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty>>,
    #[builder(into)]
    #[serde(rename = "leftOperand")]
    pub r#left_operand: Box<String>,
    #[builder(into)]
    #[serde(rename = "onClause")]
    pub r#on_clause: Box<String>,
    #[builder(into)]
    #[serde(rename = "rightJoinKeyProperties")]
    pub r#right_join_key_properties: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty>>,
    #[builder(into)]
    #[serde(rename = "rightOperand")]
    pub r#right_operand: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
