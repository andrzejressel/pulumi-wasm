#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttribute {
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "nameFormat")]
    pub r#name_format: Box<Option<String>>,
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<crate::types::AccessApplicationSaasAppCustomAttributeSource>,
}
