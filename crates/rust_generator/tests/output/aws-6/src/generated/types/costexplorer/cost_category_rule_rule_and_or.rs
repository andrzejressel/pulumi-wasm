#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CostCategoryRuleRuleAndOr {
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into, default)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndOrCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndOrDimension>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleAndOrTags>>,
}
