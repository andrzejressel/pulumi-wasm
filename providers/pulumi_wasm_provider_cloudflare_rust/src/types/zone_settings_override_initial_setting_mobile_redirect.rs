#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ZoneSettingsOverrideInitialSettingMobileRedirect {
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: Box<String>,
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[serde(rename = "stripUri")]
    pub r#strip_uri: Box<bool>,
}
