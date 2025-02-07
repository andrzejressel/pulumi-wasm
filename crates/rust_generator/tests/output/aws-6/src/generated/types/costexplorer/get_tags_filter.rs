#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTagsFilter {
    /// Return results that match both `Dimension` objects.
    #[builder(into, default)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Option<Vec<super::super::types::costexplorer::GetTagsFilterAnd>>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See `cost_category` block below for details.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::GetTagsFilterCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See `dimension` block below for details.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::GetTagsFilterDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "not")]
    pub r#not: Box<Option<super::super::types::costexplorer::GetTagsFilterNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Option<Vec<super::super::types::costexplorer::GetTagsFilterOr>>>,
    /// Tags that match your request.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::GetTagsFilterTags>>,
}
