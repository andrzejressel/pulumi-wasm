#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRealtimeLogConfigEndpoint {
    /// (Required) Amazon Kinesis data stream configuration.
    #[builder(into)]
    #[serde(rename = "kinesisStreamConfigs")]
    pub r#kinesis_stream_configs: Box<Vec<super::super::types::cloudfront::GetRealtimeLogConfigEndpointKinesisStreamConfig>>,
    /// (Required) Type of data stream where real-time log data is sent. The only valid value is `Kinesis`.
    #[builder(into)]
    #[serde(rename = "streamType")]
    pub r#stream_type: Box<String>,
}
