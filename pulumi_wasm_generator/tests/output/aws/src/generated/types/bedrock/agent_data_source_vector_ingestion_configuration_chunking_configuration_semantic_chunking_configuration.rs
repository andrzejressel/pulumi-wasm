#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration {
    /// The dissimilarity threshold for splitting chunks.
    #[builder(into)]
    #[serde(rename = "breakpointPercentileThreshold")]
    pub r#breakpoint_percentile_threshold: Box<f64>,
    /// The buffer size.
    #[builder(into)]
    #[serde(rename = "bufferSize")]
    pub r#buffer_size: Box<f64>,
    /// The maximum number of tokens a chunk can contain.
    #[builder(into)]
    #[serde(rename = "maxToken")]
    pub r#max_token: Box<f64>,
}
