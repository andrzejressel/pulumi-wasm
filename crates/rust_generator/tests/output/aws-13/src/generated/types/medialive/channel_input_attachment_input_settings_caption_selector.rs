#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelector {
    #[builder(into, default)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<Option<String>>,
    /// Name of the Channel.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "selectorSettings")]
    pub r#selector_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings>>,
}
