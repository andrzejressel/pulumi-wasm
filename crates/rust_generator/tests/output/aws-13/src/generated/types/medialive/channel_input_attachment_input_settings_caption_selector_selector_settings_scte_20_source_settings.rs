#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings {
    #[builder(into, default)]
    #[serde(rename = "convert608To708")]
    pub r#convert_608_to_708: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "source608ChannelNumber")]
    pub r#source_608_channel_number: Box<Option<i32>>,
}
