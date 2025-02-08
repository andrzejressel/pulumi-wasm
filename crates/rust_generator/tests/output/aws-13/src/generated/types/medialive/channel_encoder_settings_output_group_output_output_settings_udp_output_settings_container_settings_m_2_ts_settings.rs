#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettings {
    #[builder(into, default)]
    #[serde(rename = "absentInputAudioBehavior")]
    pub r#absent_input_audio_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "arib")]
    pub r#arib: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "aribCaptionsPid")]
    pub r#arib_captions_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "aribCaptionsPidControl")]
    pub r#arib_captions_pid_control: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "audioBufferModel")]
    pub r#audio_buffer_model: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "audioFramesPerPes")]
    pub r#audio_frames_per_pes: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "audioPids")]
    pub r#audio_pids: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "audioStreamType")]
    pub r#audio_stream_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "bufferModel")]
    pub r#buffer_model: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ccDescriptor")]
    pub r#cc_descriptor: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "dvbNitSettings")]
    pub r#dvb_nit_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbNitSettings>>,
    #[builder(into, default)]
    #[serde(rename = "dvbSdtSettings")]
    pub r#dvb_sdt_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings>>,
    #[builder(into, default)]
    #[serde(rename = "dvbSubPids")]
    pub r#dvb_sub_pids: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "dvbTdtSettings")]
    pub r#dvb_tdt_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbTdtSettings>>,
    #[builder(into, default)]
    #[serde(rename = "dvbTeletextPid")]
    pub r#dvb_teletext_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ebif")]
    pub r#ebif: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ebpAudioInterval")]
    pub r#ebp_audio_interval: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ebpLookaheadMs")]
    pub r#ebp_lookahead_ms: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "ebpPlacement")]
    pub r#ebp_placement: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ecmPid")]
    pub r#ecm_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "esRateInPes")]
    pub r#es_rate_in_pes: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "etvPlatformPid")]
    pub r#etv_platform_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "etvSignalPid")]
    pub r#etv_signal_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "fragmentTime")]
    pub r#fragment_time: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "klv")]
    pub r#klv: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "klvDataPids")]
    pub r#klv_data_pids: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "nullPacketBitrate")]
    pub r#null_packet_bitrate: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "patInterval")]
    pub r#pat_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "pcrControl")]
    pub r#pcr_control: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "pcrPeriod")]
    pub r#pcr_period: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "pcrPid")]
    pub r#pcr_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "pmtInterval")]
    pub r#pmt_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "pmtPid")]
    pub r#pmt_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "programNum")]
    pub r#program_num: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "rateMode")]
    pub r#rate_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "scte27Pids")]
    pub r#scte_27_pids: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "scte35Control")]
    pub r#scte_35_control: Box<Option<String>>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into, default)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentationMarkers")]
    pub r#segmentation_markers: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentationStyle")]
    pub r#segmentation_style: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentationTime")]
    pub r#segmentation_time: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "timedMetadataBehavior")]
    pub r#timed_metadata_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timedMetadataPid")]
    pub r#timed_metadata_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "videoPid")]
    pub r#video_pid: Box<Option<String>>,
}
