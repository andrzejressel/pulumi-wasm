#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditions {
    /// A collection of conditions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditionsCondition>>>,
}
