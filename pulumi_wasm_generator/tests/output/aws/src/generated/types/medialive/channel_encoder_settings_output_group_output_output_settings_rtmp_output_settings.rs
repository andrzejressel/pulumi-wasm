#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettings {
    /// Setting to allow self signed or verified RTMP certificates.
    #[builder(into, default)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Box<Option<String>>,
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into, default)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Box<Option<i32>>,
    /// The RTMP endpoint excluding the stream name. See Destination for more details.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsRtmpOutputSettingsDestination>,
    /// Number of retry attempts.
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
}
