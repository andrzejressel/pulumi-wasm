#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourceMappingDestinationConfig {
    /// The destination configuration for failed invocations. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "onFailure")]
    pub r#on_failure: Box<Option<super::super::types::lambda::EventSourceMappingDestinationConfigOnFailure>>,
}
