#[derive(serde::Serialize)]
pub struct AccessOrganizationLoginDesign {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}
