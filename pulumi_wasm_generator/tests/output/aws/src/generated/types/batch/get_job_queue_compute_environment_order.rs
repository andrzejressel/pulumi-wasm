#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobQueueComputeEnvironmentOrder {
    #[builder(into)]
    #[serde(rename = "computeEnvironment")]
    pub r#compute_environment: Box<String>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
}