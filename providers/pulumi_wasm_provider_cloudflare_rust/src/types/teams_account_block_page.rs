#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountBlockPage {
    /// Hex code of block page background color.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// Indicator of enablement.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Block page footer text.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// Block page header text.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// URL of block page logo.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// Admin email for users to contact.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    /// Subject line for emails created from block page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    /// Name of block page configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
