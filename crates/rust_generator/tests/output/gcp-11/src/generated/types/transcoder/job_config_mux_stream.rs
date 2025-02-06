#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfigMuxStream {
    /// The container format. The default is `mp4`.
    #[builder(into, default)]
    #[serde(rename = "container")]
    pub r#container: Box<Option<String>>,
    /// List of ElementaryStream.key values multiplexed in this stream.
    #[builder(into, default)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Box<Option<Vec<String>>>,
    /// Identifier of the encryption configuration to use.
    #[builder(into, default)]
    #[serde(rename = "encryptionId")]
    pub r#encryption_id: Box<Option<String>>,
    /// The name of the generated file.
    #[builder(into, default)]
    #[serde(rename = "fileName")]
    pub r#file_name: Box<Option<String>>,
    /// A unique key for this multiplexed stream.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Segment settings for ts, fmp4 and vtt.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "segmentSettings")]
    pub r#segment_settings: Box<Option<super::super::types::transcoder::JobConfigMuxStreamSegmentSettings>>,
}
