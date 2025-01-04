#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateConfigElementaryStreamAudioStream {
    /// Audio bitrate in bits per second.
    #[builder(into)]
    #[serde(rename = "bitrateBps")]
    pub r#bitrate_bps: Box<i32>,
    /// Number of audio channels. The default is `2`.
    #[builder(into, default)]
    #[serde(rename = "channelCount")]
    pub r#channel_count: Box<Option<i32>>,
    /// A list of channel names specifying layout of the audio channels.  The default is ["fl", "fr"].
    #[builder(into, default)]
    #[serde(rename = "channelLayouts")]
    pub r#channel_layouts: Box<Option<Vec<String>>>,
    /// The codec for this audio stream. The default is `aac`.
    #[builder(into, default)]
    #[serde(rename = "codec")]
    pub r#codec: Box<Option<String>>,
    /// The audio sample rate in Hertz. The default is `48000`.
    #[builder(into, default)]
    #[serde(rename = "sampleRateHertz")]
    pub r#sample_rate_hertz: Box<Option<i32>>,
}
