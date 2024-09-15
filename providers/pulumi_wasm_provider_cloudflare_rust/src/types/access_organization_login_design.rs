#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// The text at the bottom of the login page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// The text at the top of the login page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// The URL of the logo on the login page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// The text color on the login page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}
