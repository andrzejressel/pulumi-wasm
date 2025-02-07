#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings {
    #[builder(into, default)]
    #[serde(rename = "temporalFilterSettings")]
    pub r#temporal_filter_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings>>,
}
