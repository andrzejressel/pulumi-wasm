#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfigElementaryStream {
    /// Encoding of an audio stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "audioStream")]
    pub r#audio_stream: Box<Option<super::super::types::transcoder::JobConfigElementaryStreamAudioStream>>,
    /// A unique key for this atom.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Encoding of a video stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "videoStream")]
    pub r#video_stream: Box<Option<super::super::types::transcoder::JobConfigElementaryStreamVideoStream>>,
}
