#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupVariationImageResponseCard {
    /// Configuration blocks for buttons that should be displayed on the response card. The arrangement of the buttons is determined by the platform that displays the button. See `button`.
    #[builder(into, default)]
    #[serde(rename = "buttons")]
    pub r#buttons: Box<Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalConditionalBranchResponseMessageGroupVariationImageResponseCardButton>>>,
    /// URL of an image to display on the response card. The image URL must be publicly available so that the platform displaying the response card has access to the image.
    #[builder(into, default)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    /// Subtitle to display on the response card. The format of the subtitle is determined by the platform displaying the response card.
    #[builder(into, default)]
    #[serde(rename = "subtitle")]
    pub r#subtitle: Box<Option<String>>,
    /// Title to display on the response card. The format of the title is determined by the platform displaying the response card.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
