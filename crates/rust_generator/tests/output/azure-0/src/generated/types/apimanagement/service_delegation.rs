#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceDelegation {
    /// Should subscription requests be delegated to an external url? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "subscriptionsEnabled")]
    pub r#subscriptions_enabled: Box<Option<bool>>,
    /// The delegation URL.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// Should user registration requests be delegated to an external url? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "userRegistrationEnabled")]
    pub r#user_registration_enabled: Box<Option<bool>>,
    /// A base64-encoded validation key to validate, that a request is coming from Azure API Management.
    #[builder(into, default)]
    #[serde(rename = "validationKey")]
    pub r#validation_key: Box<Option<String>>,
}
