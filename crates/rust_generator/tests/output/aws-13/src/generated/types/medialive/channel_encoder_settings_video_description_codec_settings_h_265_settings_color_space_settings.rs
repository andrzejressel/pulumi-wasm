#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings {
    /// Sets the colorspace metadata to be passed through.
    #[builder(into, default)]
    #[serde(rename = "colorSpacePassthroughSettings")]
    pub r#color_space_passthrough_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsColorSpacePassthroughSettings>>,
    /// Set the colorspace to Dolby Vision81.
    #[builder(into, default)]
    #[serde(rename = "dolbyVision81Settings")]
    pub r#dolby_vision_81_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsDolbyVision81Settings>>,
    /// Set the colorspace to be HDR10. See H265 HDR10 Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "hdr10Settings")]
    pub r#hdr_10_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings>>,
    /// Set the colorspace to Rec. 601.
    #[builder(into, default)]
    #[serde(rename = "rec601Settings")]
    pub r#rec_601_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec601Settings>>,
    /// Set the colorspace to Rec. 709.
    #[builder(into, default)]
    #[serde(rename = "rec709Settings")]
    pub r#rec_709_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsRec709Settings>>,
}
