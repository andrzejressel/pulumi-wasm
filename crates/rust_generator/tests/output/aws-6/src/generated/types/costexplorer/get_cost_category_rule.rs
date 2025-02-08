#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRule {
    /// Configuration block for the value the line item is categorized as if the line item contains the matched dimension. See below.
    #[builder(into)]
    #[serde(rename = "inheritedValues")]
    pub r#inherited_values: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleInheritedValue>>,
    /// Configuration block for the `Expression` object used to categorize costs. See below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::costexplorer::GetCostCategoryRuleRule>>,
    /// Parameter type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Default value for the cost category.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
