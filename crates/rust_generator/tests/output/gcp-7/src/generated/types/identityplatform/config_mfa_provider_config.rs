#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigMfaProviderConfig {
    /// Whether MultiFactor Authentication has been enabled for this project.
    /// Possible values are: `DISABLED`, `ENABLED`, `MANDATORY`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// TOTP MFA provider config for this project.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "totpProviderConfig")]
    pub r#totp_provider_config: Box<Option<super::super::types::identityplatform::ConfigMfaProviderConfigTotpProviderConfig>>,
}
