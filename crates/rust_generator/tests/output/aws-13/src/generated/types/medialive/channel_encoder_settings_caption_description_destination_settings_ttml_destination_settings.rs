#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings {
    /// This field is not currently supported and will not affect the output styling. Leave the default value.
    #[builder(into)]
    #[serde(rename = "styleControl")]
    pub r#style_control: Box<String>,
}
