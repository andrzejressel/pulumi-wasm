#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettings {
    #[builder(into)]
    #[serde(rename = "channelMappings")]
    pub r#channel_mappings: Box<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping>>,
    #[builder(into, default)]
    #[serde(rename = "channelsIn")]
    pub r#channels_in: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "channelsOut")]
    pub r#channels_out: Box<Option<i32>>,
}
