#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationRuntimeSsisPipelineExternalComputeScale {
    /// Specifies the number of the external nodes, which should be greater than `0` and less than `11`.
    #[builder(into, default)]
    #[serde(rename = "numberOfExternalNodes")]
    pub r#number_of_external_nodes: Box<Option<i32>>,
    /// Specifies the number of the pipeline nodes, which should be greater than `0` and less than `11`.
    #[builder(into, default)]
    #[serde(rename = "numberOfPipelineNodes")]
    pub r#number_of_pipeline_nodes: Box<Option<i32>>,
    /// Specifies the time to live (in minutes) setting of integration runtime which will execute copy activity. Possible values are at least `5`.
    #[builder(into, default)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Box<Option<i32>>,
}
