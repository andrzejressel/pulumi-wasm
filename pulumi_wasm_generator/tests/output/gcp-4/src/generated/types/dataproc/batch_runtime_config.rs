#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BatchRuntimeConfig {
    /// Optional. Autotuning configuration of the workload.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autotuningConfig")]
    pub r#autotuning_config: Box<Option<super::super::types::dataproc::BatchRuntimeConfigAutotuningConfig>>,
    /// Optional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs.
    #[builder(into, default)]
    #[serde(rename = "cohort")]
    pub r#cohort: Box<Option<String>>,
    /// Optional custom container image for the job runtime environment. If not specified, a default container image will be used.
    #[builder(into, default)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Box<Option<String>>,
    /// (Output)
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into, default)]
    #[serde(rename = "effectiveProperties")]
    pub r#effective_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Version of the batch runtime.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
