#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CostCategoryRuleRule {
    /// Return results that match both `Dimension` objects.
    #[builder(into, default)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleAnd>>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "not")]
    pub r#not: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleOr>>>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleTags>>,
}
