#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutput {
    /// The names of the audio descriptions used as audio sources for the output.
    #[builder(into, default)]
    #[serde(rename = "audioDescriptionNames")]
    pub r#audio_description_names: Box<Option<Vec<String>>>,
    /// The names of the caption descriptions used as caption sources for the output.
    #[builder(into, default)]
    #[serde(rename = "captionDescriptionNames")]
    pub r#caption_description_names: Box<Option<Vec<String>>>,
    /// The name used to identify an output.
    #[builder(into, default)]
    #[serde(rename = "outputName")]
    pub r#output_name: Box<Option<String>>,
    /// Settings for output. See Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "outputSettings")]
    pub r#output_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettings>,
    /// The name of the video description used as video source for the output.
    #[builder(into, default)]
    #[serde(rename = "videoDescriptionName")]
    pub r#video_description_name: Box<Option<String>>,
}
