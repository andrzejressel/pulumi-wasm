#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SettingsAccessSettingsWorkforceIdentitySettings {
    /// OAuth 2.0 settings for IAP to perform OIDC flow with workforce identity
    /// federation services.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_oauth2"></a>The `oauth2` block supports:
    #[builder(into, default)]
    #[serde(rename = "oauth2")]
    pub r#oauth_2: Box<Option<super::super::types::iap::SettingsAccessSettingsWorkforceIdentitySettingsOauth2>>,
    /// The workforce pool resources. Only one workforce pool is accepted.
    #[builder(into, default)]
    #[serde(rename = "workforcePools")]
    pub r#workforce_pools: Box<Option<String>>,
}
