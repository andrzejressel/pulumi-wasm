#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SettingsAccessSettings {
    /// Settings to configure and enable allowed domains.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowedDomainsSettings")]
    pub r#allowed_domains_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsAllowedDomainsSettings>>,
    /// Configuration to allow cross-origin requests via IAP.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "corsSettings")]
    pub r#cors_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsCorsSettings>>,
    /// GCIP claims and endpoint configurations for 3p identity providers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcipSettings")]
    pub r#gcip_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsGcipSettings>>,
    /// Identity sources that IAP can use to authenticate the end user. Only one identity source
    /// can be configured. The possible values are:
    /// * `WORKFORCE_IDENTITY_FEDERATION`: Use external identities set up on Google Cloud Workforce
    /// Identity Federation.
    /// Each value may be one of: `WORKFORCE_IDENTITY_FEDERATION`.
    #[builder(into, default)]
    #[serde(rename = "identitySources")]
    pub r#identity_sources: Box<Option<Vec<String>>>,
    /// Settings to configure IAP's OAuth behavior.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauthSettings")]
    pub r#oauth_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsOauthSettings>>,
    /// Settings to configure reauthentication policies in IAP.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "reauthSettings")]
    pub r#reauth_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsReauthSettings>>,
    /// Settings to configure the workforce identity federation, including workforce pools
    /// and OAuth 2.0 settings.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "workforceIdentitySettings")]
    pub r#workforce_identity_settings: Box<Option<super::super::types::iap::SettingsAccessSettingsWorkforceIdentitySettings>>,
}
