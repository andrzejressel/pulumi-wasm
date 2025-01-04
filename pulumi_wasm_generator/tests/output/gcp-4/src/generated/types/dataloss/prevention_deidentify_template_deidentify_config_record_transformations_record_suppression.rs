#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsRecordSuppression {
    /// A condition that when it evaluates to true will result in the record being evaluated to be suppressed from the transformed content.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsRecordSuppressionCondition>>,
}
