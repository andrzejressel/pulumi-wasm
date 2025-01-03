#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTagsFilterNot {
    /// Configuration block for the filter that's based on `CostCategory` values. See `cost_category` block below for details.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::GetTagsFilterNotCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See `dimension` block below for details.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::GetTagsFilterNotDimension>>,
    /// Tags that match your request.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::GetTagsFilterNotTags>>,
}
