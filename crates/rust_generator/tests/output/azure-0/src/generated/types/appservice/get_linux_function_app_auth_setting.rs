#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxFunctionAppAuthSetting {
    /// An `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingActiveDirectory>>,
    /// A map of login parameters sent to the OpenID Connect authorization endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: Box<std::collections::HashMap<String, String>>,
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Box<Vec<String>>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Box<String>,
    /// Is this backup job enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebooks")]
    pub r#facebooks: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingFacebook>>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingGithub>>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "googles")]
    pub r#googles: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingGoogle>>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Linux Web App.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsofts")]
    pub r#microsofts: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingMicrosoft>>,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<String>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: Box<f64>,
    /// Is the Token Store configuration Enabled.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Box<bool>,
    /// A `twitter` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitters")]
    pub r#twitters: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppAuthSettingTwitter>>,
    /// The action to taken when an unauthenticated client attempts to access the app.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: Box<String>,
}
