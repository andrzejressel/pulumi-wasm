#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCostCategoryRuleRuleOr {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrAnd>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategories")]
    pub r#cost_categories: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrDimension>>,
    /// Return results that do not match the `Dimension` object.
    #[builder(into)]
    #[serde(rename = "nots")]
    pub r#nots: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrNot>>,
    /// Return results that match either `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrOr>>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrTag>>,
}
