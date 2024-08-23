#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsMinify {
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}
