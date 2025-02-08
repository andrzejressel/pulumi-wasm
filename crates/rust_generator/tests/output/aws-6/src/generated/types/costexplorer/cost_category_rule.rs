#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CostCategoryRule {
    /// Configuration block for the value the line item is categorized as if the line item contains the matched dimension. See below.
    #[builder(into, default)]
    #[serde(rename = "inheritedValue")]
    pub r#inherited_value: Box<Option<super::super::types::costexplorer::CostCategoryRuleInheritedValue>>,
    /// Configuration block for the `Expression` object used to categorize costs. See below.
    #[builder(into, default)]
    #[serde(rename = "rule")]
    pub r#rule: Box<Option<super::super::types::costexplorer::CostCategoryRuleRule>>,
    /// You can define the CostCategoryRule rule type as either `REGULAR` or `INHERITED_VALUE`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// Default value for the cost category.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
