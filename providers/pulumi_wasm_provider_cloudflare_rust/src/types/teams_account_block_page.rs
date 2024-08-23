#[derive(serde::Serialize)]
pub struct TeamsAccountBlockPage {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
