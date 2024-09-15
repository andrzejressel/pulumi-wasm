#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideInitialSettingSecurityHeader {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "nosniff")]
    pub r#nosniff: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}
