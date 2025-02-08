#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapSourceJoinInstruction {
    /// Join key properties of the left operand. See left_join_key_properties.
    #[builder(into, default)]
    #[serde(rename = "leftJoinKeyProperties")]
    pub r#left_join_key_properties: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperties>>,
    /// Operand on the left side of a join.
    #[builder(into)]
    #[serde(rename = "leftOperand")]
    pub r#left_operand: Box<String>,
    /// Join instructions provided in the ON clause of a join.
    #[builder(into)]
    #[serde(rename = "onClause")]
    pub r#on_clause: Box<String>,
    /// Join key properties of the right operand. See right_join_key_properties.
    #[builder(into, default)]
    #[serde(rename = "rightJoinKeyProperties")]
    pub r#right_join_key_properties: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperties>>,
    /// Operand on the right side of a join.
    #[builder(into)]
    #[serde(rename = "rightOperand")]
    pub r#right_operand: Box<String>,
    /// Type of join. Valid values are `INNER`, `OUTER`, `LEFT`, and `RIGHT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
