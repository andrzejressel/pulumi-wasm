#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel {
    #[builder(into)]
    #[serde(rename = "gain")]
    pub r#gain: Box<i32>,
    #[builder(into)]
    #[serde(rename = "inputChannel")]
    pub r#input_channel: Box<i32>,
}
