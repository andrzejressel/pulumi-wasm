#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigMfa {
    /// A list of usable second factors for this project.
    /// Each value may be one of: `PHONE_SMS`.
    #[builder(into, default)]
    #[serde(rename = "enabledProviders")]
    pub r#enabled_providers: Box<Option<Vec<String>>>,
    /// A list of usable second factors for this project along with their configurations.
    /// This field does not support phone based MFA, for that use the 'enabledProviders' field.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "providerConfigs")]
    pub r#provider_configs: Box<Option<Vec<super::super::types::identityplatform::ConfigMfaProviderConfig>>>,
    /// Whether MultiFactor Authentication has been enabled for this project.
    /// Possible values are: `DISABLED`, `ENABLED`, `MANDATORY`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
