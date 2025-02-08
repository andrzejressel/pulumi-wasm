#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRuleRuleNot {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotAnd>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategories")]
    pub r#cost_categories: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotDimension>>,
    /// Return results that do not match the `Dimension` object.
    #[builder(into)]
    #[serde(rename = "nots")]
    pub r#nots: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotNot>>,
    /// Return results that match either `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotOr>>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleNotTag>>,
}
