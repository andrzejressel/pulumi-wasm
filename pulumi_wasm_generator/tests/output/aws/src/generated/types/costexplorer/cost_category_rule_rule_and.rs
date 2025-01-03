#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CostCategoryRuleRuleAnd {
    /// Return results that match both `Dimension` objects.
    #[builder(into, default)]
    #[serde(rename = "ands")]
    pub r#ands: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleAndAnd>>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "not")]
    pub r#not: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into, default)]
    #[serde(rename = "ors")]
    pub r#ors: Box<Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleAndOr>>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndTags>>,
}
