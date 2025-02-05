#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeTargetParametersCloudwatchLogsParameters {
    /// The name of the log stream.
    #[builder(into, default)]
    #[serde(rename = "logStreamName")]
    pub r#log_stream_name: Box<Option<String>>,
    /// The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. This is the JSON path to the field in the event e.g. $.detail.timestamp
    #[builder(into, default)]
    #[serde(rename = "timestamp")]
    pub r#timestamp: Box<Option<String>>,
}
