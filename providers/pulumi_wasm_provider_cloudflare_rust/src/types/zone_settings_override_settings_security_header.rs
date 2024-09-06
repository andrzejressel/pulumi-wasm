#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ZoneSettingsOverrideSettingsSecurityHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    #[serde(rename = "nosniff")]
    pub r#nosniff: Box<Option<bool>>,
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}
