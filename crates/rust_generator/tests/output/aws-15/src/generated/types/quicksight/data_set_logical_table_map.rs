#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetLogicalTableMap {
    /// A display name for the logical table.
    #[builder(into)]
    #[serde(rename = "alias")]
    pub r#alias: Box<String>,
    /// Transform operations that act on this logical table. For this structure to be valid, only one of the attributes can be non-null. See data_transforms.
    #[builder(into, default)]
    #[serde(rename = "dataTransforms")]
    pub r#data_transforms: Box<Option<Vec<super::super::types::quicksight::DataSetLogicalTableMapDataTransform>>>,
    /// Key of the logical table map.
    #[builder(into)]
    #[serde(rename = "logicalTableMapId")]
    pub r#logical_table_map_id: Box<String>,
    /// Source of this logical table. See source.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::quicksight::DataSetLogicalTableMapSource>,
}
