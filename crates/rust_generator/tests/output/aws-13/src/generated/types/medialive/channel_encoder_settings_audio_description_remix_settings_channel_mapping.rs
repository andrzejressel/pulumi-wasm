#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping {
    #[builder(into)]
    #[serde(rename = "inputChannelLevels")]
    pub r#input_channel_levels: Box<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMappingInputChannelLevel>>,
    #[builder(into)]
    #[serde(rename = "outputChannel")]
    pub r#output_channel: Box<i32>,
}
