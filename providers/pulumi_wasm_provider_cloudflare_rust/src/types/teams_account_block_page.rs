#[derive(serde::Serialize)]
pub struct TeamsAccountBlockPage {
    /// Hex code of block page background color.
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// Indicator of enablement.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Block page footer text.
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// Block page header text.
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// URL of block page logo.
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// Admin email for users to contact.
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    /// Subject line for emails created from block page.
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    /// Name of block page configuration.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
