#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsAvailBlanking {
    /// Blanking image to be used. See Avail Blanking Image for more details.
    #[builder(into, default)]
    #[serde(rename = "availBlankingImage")]
    pub r#avail_blanking_image: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAvailBlankingAvailBlankingImage>>,
    /// When set to enabled, causes video, audio and captions to be blanked when insertion metadata is added.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
