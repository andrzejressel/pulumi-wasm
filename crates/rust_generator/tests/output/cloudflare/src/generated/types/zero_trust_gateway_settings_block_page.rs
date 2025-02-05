#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustGatewaySettingsBlockPage {
    /// Hex code of block page background color.
    #[builder(into, default)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// Indicator of enablement.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Block page footer text.
    #[builder(into, default)]
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// Block page header text.
    #[builder(into, default)]
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// URL of block page logo.
    #[builder(into, default)]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// Admin email for users to contact.
    #[builder(into, default)]
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    /// Subject line for emails created from block page.
    #[builder(into, default)]
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    /// Name of block page configuration.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
