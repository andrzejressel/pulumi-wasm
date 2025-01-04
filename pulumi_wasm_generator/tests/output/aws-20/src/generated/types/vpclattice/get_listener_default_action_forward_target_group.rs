#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerDefaultActionForwardTargetGroup {
    #[builder(into)]
    #[serde(rename = "targetGroupIdentifier")]
    pub r#target_group_identifier: Box<String>,
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
