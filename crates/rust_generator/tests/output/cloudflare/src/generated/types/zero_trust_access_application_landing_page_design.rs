#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZeroTrustAccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[builder(into, default)]
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    /// The button text color of the landing page.
    #[builder(into, default)]
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    /// The URL of the image to be displayed in the landing page.
    #[builder(into, default)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    /// The message of the landing page.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// The title of the landing page.
    #[builder(into, default)]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
