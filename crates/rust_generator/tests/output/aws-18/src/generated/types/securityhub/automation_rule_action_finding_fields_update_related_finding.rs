#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRuleActionFindingFieldsUpdateRelatedFinding {
    /// The product-generated identifier for a related finding.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ARN of the product that generated a related finding.
    #[builder(into)]
    #[serde(rename = "productArn")]
    pub r#product_arn: Box<String>,
}
