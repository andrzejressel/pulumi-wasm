#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsVideoSelector {
    #[builder(into, default)]
    #[serde(rename = "colorSpace")]
    pub r#color_space: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "colorSpaceUsage")]
    pub r#color_space_usage: Box<Option<String>>,
}
