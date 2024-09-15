#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideInitialSettingMinify {
    #[builder(into)]
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    #[builder(into)]
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}
