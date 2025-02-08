#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings {
    /// The bitrate is specified in bits per second, as in an HLS manifest.
    #[builder(into, default)]
    #[serde(rename = "bandwidth")]
    pub r#bandwidth: Box<Option<i32>>,
    /// Buffer segments.
    #[builder(into, default)]
    #[serde(rename = "bufferSegments")]
    pub r#buffer_segments: Box<Option<i32>>,
    /// The number of consecutive times that attempts to read a manifest or segment must fail before the input is considered unavailable.
    #[builder(into, default)]
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    /// The number of seconds between retries when an attempt to read a manifest or segment fails.
    #[builder(into, default)]
    #[serde(rename = "retryInterval")]
    pub r#retry_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "scte35Source")]
    pub r#scte_35_source: Box<Option<String>>,
}
