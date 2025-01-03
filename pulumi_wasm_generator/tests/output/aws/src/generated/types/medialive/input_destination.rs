#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InputDestination {
    /// A unique name for the location the RTMP stream is being pushed to.
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<String>,
}
