#[derive(serde::Serialize)]
pub struct AccessApplicationLandingPageDesign {
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
