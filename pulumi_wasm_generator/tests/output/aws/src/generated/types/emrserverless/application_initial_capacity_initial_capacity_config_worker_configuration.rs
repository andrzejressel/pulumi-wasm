#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationInitialCapacityInitialCapacityConfigWorkerConfiguration {
    /// The CPU requirements for every worker instance of the worker type.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<String>,
    /// The disk requirements for every worker instance of the worker type.
    #[builder(into, default)]
    #[serde(rename = "disk")]
    pub r#disk: Box<Option<String>>,
    /// The memory requirements for every worker instance of the worker type.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Box<String>,
}