#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlanStage {
    #[builder(into)]
    #[serde(rename = "durationInMinutes")]
    pub r#duration_in_minutes: Box<i32>,
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Vec<super::super::types::ssmcontacts::GetPlanStageTarget>>,
}
