#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputSpecification {
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Box<String>,
    #[builder(into)]
    #[serde(rename = "inputResolution")]
    pub r#input_resolution: Box<String>,
    #[builder(into)]
    #[serde(rename = "maximumBitrate")]
    pub r#maximum_bitrate: Box<String>,
}