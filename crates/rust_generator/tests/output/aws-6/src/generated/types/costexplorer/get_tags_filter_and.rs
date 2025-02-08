#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTagsFilterAnd {
    /// Configuration block for the filter that's based on `CostCategory` values. See `cost_category` block below for details.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::GetTagsFilterAndCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See `dimension` block below for details.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::GetTagsFilterAndDimension>>,
    /// Tags that match your request.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::GetTagsFilterAndTags>>,
}
