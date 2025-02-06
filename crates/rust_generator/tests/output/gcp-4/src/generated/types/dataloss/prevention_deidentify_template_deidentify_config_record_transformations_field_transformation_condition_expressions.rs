#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressions {
    /// Conditions to apply to the expression.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditions>>,
    /// The operator to apply to the result of conditions. Default and currently only supported value is AND.
    /// Default value is `AND`.
    /// Possible values are: `AND`.
    #[builder(into, default)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Box<Option<String>>,
}
