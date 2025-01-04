#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsVideoDescription {
    /// The video codec settings. See Video Codec Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "codecSettings")]
    pub r#codec_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettings>>,
    /// Output video height in pixels.
    #[builder(into, default)]
    #[serde(rename = "height")]
    pub r#height: Box<Option<i32>>,
    /// The name of the video description.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Indicate how to respond to the AFD values that might be in the input video.
    #[builder(into, default)]
    #[serde(rename = "respondToAfd")]
    pub r#respond_to_afd: Box<Option<String>>,
    /// Behavior on how to scale.
    #[builder(into, default)]
    #[serde(rename = "scalingBehavior")]
    pub r#scaling_behavior: Box<Option<String>>,
    /// Changes the strength of the anti-alias filter used for scaling.
    #[builder(into, default)]
    #[serde(rename = "sharpness")]
    pub r#sharpness: Box<Option<i32>>,
    /// Output video width in pixels.
    #[builder(into, default)]
    #[serde(rename = "width")]
    pub r#width: Box<Option<i32>>,
}
