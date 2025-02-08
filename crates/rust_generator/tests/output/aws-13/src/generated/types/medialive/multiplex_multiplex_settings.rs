#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MultiplexMultiplexSettings {
    /// Maximum video buffer delay.
    #[builder(into, default)]
    #[serde(rename = "maximumVideoBufferDelayMilliseconds")]
    pub r#maximum_video_buffer_delay_milliseconds: Box<Option<i32>>,
    /// Transport stream bit rate.
    #[builder(into)]
    #[serde(rename = "transportStreamBitrate")]
    pub r#transport_stream_bitrate: Box<i32>,
    /// Unique ID for each multiplex.
    #[builder(into)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: Box<i32>,
    /// Transport stream reserved bit rate.
    #[builder(into, default)]
    #[serde(rename = "transportStreamReservedBitrate")]
    pub r#transport_stream_reserved_bitrate: Box<Option<i32>>,
}
