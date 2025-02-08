#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccessApplicationSaasAppCustomAttribute {
    /// A friendly name for the attribute as provided to the SaaS app.
    #[builder(into, default)]
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Box<Option<String>>,
    /// The name of the attribute as provided to the SaaS app.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A globally unique name for an identity or service provider.
    #[builder(into, default)]
    #[serde(rename = "nameFormat")]
    pub r#name_format: Box<Option<String>>,
    /// True if the attribute must be always present.
    #[builder(into, default)]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::types::AccessApplicationSaasAppCustomAttributeSource>,
}
