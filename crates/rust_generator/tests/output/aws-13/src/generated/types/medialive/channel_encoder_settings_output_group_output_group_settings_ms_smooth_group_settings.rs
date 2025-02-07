#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
    #[builder(into, default)]
    #[serde(rename = "acquisitionPointId")]
    pub r#acquisition_point_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "audioOnlyTimecodeControl")]
    pub r#audio_only_timecode_control: Box<Option<String>>,
    /// Setting to allow self signed or verified RTMP certificates.
    #[builder(into, default)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Box<Option<String>>,
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into, default)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Box<Option<i32>>,
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination>,
    #[builder(into, default)]
    #[serde(rename = "eventId")]
    pub r#event_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "eventIdMode")]
    pub r#event_id_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "eventStopBehavior")]
    pub r#event_stop_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "fragmentLength")]
    pub r#fragment_length: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Box<Option<String>>,
    /// Number of retry attempts.
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into, default)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "segmentationMode")]
    pub r#segmentation_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sendDelayMs")]
    pub r#send_delay_ms: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "sparseTrackType")]
    pub r#sparse_track_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "streamManifestBehavior")]
    pub r#stream_manifest_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timestampOffset")]
    pub r#timestamp_offset: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timestampOffsetMode")]
    pub r#timestamp_offset_mode: Box<Option<String>>,
}
