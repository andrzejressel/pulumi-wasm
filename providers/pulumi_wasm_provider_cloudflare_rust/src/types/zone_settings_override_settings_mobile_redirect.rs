#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideSettingsMobileRedirect {
    #[builder(into)]
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[builder(into)]
    #[serde(rename = "stripUri")]
    pub r#strip_uri: Box<bool>,
}
