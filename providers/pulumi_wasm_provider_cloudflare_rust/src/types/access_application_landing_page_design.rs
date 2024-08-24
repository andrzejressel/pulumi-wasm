#[derive(serde::Serialize)]
pub struct AccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    /// The button text color of the landing page.
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    /// The URL of the image to be displayed in the landing page.
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    /// The message of the landing page.
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// The title of the landing page.
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
