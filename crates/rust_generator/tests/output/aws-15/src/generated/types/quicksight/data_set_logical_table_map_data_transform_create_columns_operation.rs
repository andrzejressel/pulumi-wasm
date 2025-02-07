#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetLogicalTableMapDataTransformCreateColumnsOperation {
    /// Calculated columns to create. See columns.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::quicksight::DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn>>,
}
