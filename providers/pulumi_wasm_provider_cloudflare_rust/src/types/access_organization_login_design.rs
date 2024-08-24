#[derive(serde::Serialize)]
pub struct AccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// The text at the bottom of the login page.
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// The text at the top of the login page.
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// The URL of the logo on the login page.
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// The text color on the login page.
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}
