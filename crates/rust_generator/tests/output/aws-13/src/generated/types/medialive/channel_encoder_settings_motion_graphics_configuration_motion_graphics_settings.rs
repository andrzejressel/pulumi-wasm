#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings {
    /// Html Motion Graphics Settings.
    #[builder(into, default)]
    #[serde(rename = "htmlMotionGraphicsSettings")]
    pub r#html_motion_graphics_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettingsHtmlMotionGraphicsSettings>>,
}
