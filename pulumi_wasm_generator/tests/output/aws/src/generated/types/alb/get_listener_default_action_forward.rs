#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerDefaultActionForward {
    #[builder(into)]
    #[serde(rename = "stickinesses")]
    pub r#stickinesses: Box<Vec<super::super::types::alb::GetListenerDefaultActionForwardStickiness>>,
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Vec<super::super::types::alb::GetListenerDefaultActionForwardTargetGroup>>,
}