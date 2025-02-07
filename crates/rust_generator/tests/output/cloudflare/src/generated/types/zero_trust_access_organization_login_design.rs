#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustAccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[builder(into, default)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// The text at the bottom of the login page.
    #[builder(into, default)]
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// The text at the top of the login page.
    #[builder(into, default)]
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// The URL of the logo on the login page.
    #[builder(into, default)]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// The text color on the login page.
    #[builder(into, default)]
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}
