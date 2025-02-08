#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CostCategoryRuleRuleNot {
    /// Return results that match both `Dimension` objects.
    #[builder(into, default)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleNotAnd>>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleNotCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleNotDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "not")]
    pub r#not: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleNotNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleNotOr>>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleNotTags>>,
}
