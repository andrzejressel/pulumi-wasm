#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3U8Settings {
    #[builder(into, default)]
    #[serde(rename = "audioFramesPerPes")]
    pub r#audio_frames_per_pes: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "audioPids")]
    pub r#audio_pids: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ecmPid")]
    pub r#ecm_pid: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Box<Option<String>>,
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
    #[serde(rename = "scte35Behavior")]
    pub r#scte_35_behavior: Box<Option<String>>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into, default)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Box<Option<String>>,
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
