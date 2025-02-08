#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSlotAuthSettings {
    /// An `active_directory` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Box<Option<super::super::types::appservice::FunctionAppSlotAuthSettingsActiveDirectory>>,
    /// login parameters to send to the OpenID Connect authorization endpoint when a user logs in. Each parameter must be in the form "key=value".
    #[builder(into, default)]
    #[serde(rename = "additionalLoginParams")]
    pub r#additional_login_params: Box<Option<std::collections::HashMap<String, String>>>,
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into, default)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Box<Option<Vec<String>>>,
    /// The default provider to use when multiple providers have been set up. Possible values are `AzureActiveDirectory`, `Facebook`, `Google`, `MicrosoftAccount` and `Twitter`.
    /// 
    /// > **NOTE:** When using multiple providers, the default provider must be set for settings like `unauthenticated_client_action` to work.
    #[builder(into, default)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Box<Option<String>>,
    /// Is Authentication enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `facebook` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "facebook")]
    pub r#facebook: Box<Option<super::super::types::appservice::FunctionAppSlotAuthSettingsFacebook>>,
    /// A `google` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "google")]
    pub r#google: Box<Option<super::super::types::appservice::FunctionAppSlotAuthSettingsGoogle>>,
    /// Issuer URI. When using Azure Active Directory, this value is the URI of the directory tenant, e.g. <https://sts.windows.net/{tenant-guid}/>.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// A `microsoft` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "microsoft")]
    pub r#microsoft: Box<Option<super::super::types::appservice::FunctionAppSlotAuthSettingsMicrosoft>>,
    /// The runtime version of the Authentication/Authorization module.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72`.
    #[builder(into, default)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: Box<Option<f64>>,
    /// If enabled the module will durably store platform-specific security tokens that are obtained during login flows. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Box<Option<bool>>,
    /// A `twitter` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "twitter")]
    pub r#twitter: Box<Option<super::super::types::appservice::FunctionAppSlotAuthSettingsTwitter>>,
    /// The action to take when an unauthenticated client attempts to access the app. Possible values are `AllowAnonymous` and `RedirectToLoginPage`.
    #[builder(into, default)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: Box<Option<String>>,
}
