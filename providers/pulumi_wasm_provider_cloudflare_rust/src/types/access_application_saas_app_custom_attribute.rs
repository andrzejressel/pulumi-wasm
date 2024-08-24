#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttribute {
    /// A friendly name for the attribute as provided to the SaaS app.
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Box<Option<String>>,
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A globally unique name for an identity or service provider.
    #[serde(rename = "nameFormat")]
    pub r#name_format: Box<Option<String>>,
    /// True if the attribute must be always present.
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<crate::types::AccessApplicationSaasAppCustomAttributeSource>,
}
