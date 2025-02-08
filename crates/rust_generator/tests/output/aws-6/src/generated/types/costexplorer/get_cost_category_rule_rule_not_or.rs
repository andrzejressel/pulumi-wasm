#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRuleRuleNotOr {
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategories")]
    pub r#cost_categories: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotOrCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotOrDimension>>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotOrTag>>,
}
