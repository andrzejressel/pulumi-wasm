#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsEventSource {
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsEventSourceParameters>,
    /// The source of the event. Currently only managed CloudWatch Events rules are supported. Valid values are `MANAGED_CWE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}