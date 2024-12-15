#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    /// The button text color of the landing page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    /// The URL of the image to be displayed in the landing page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    /// The message of the landing page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// The title of the landing page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
