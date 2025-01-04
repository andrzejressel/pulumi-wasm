#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsActionCrossRegionCopyRetainRule {
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    #[builder(into)]
    #[serde(rename = "intervalUnit")]
    pub r#interval_unit: Box<String>,
}
