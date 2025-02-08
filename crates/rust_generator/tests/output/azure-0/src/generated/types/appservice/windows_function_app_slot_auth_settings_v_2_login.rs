#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotAuthSettingsV2Login {
    /// External URLs that can be redirected to as part of logging in or logging out of the app. This is an advanced setting typically only needed by Windows Store application backends.
    /// 
    /// > **Note:** URLs within the current domain are always implicitly allowed.
    #[builder(into, default)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Box<Option<Vec<String>>>,
    /// The method by which cookies expire. Possible values include: `FixedTime`, and `IdentityProviderDerived`. Defaults to `FixedTime`.
    #[builder(into, default)]
    #[serde(rename = "cookieExpirationConvention")]
    pub r#cookie_expiration_convention: Box<Option<String>>,
    /// The time after the request is made when the session cookie should expire. Defaults to `08:00:00`.
    #[builder(into, default)]
    #[serde(rename = "cookieExpirationTime")]
    pub r#cookie_expiration_time: Box<Option<String>>,
    /// The endpoint to which logout requests should be made.
    #[builder(into, default)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: Box<Option<String>>,
    /// The time after the request is made when the nonce should expire. Defaults to `00:05:00`.
    #[builder(into, default)]
    #[serde(rename = "nonceExpirationTime")]
    pub r#nonce_expiration_time: Box<Option<String>>,
    /// Should the fragments from the request be preserved after the login request is made. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "preserveUrlFragmentsForLogins")]
    pub r#preserve_url_fragments_for_logins: Box<Option<bool>>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into, default)]
    #[serde(rename = "tokenRefreshExtensionTime")]
    pub r#token_refresh_extension_time: Box<Option<f64>>,
    /// Should the Token Store configuration Enabled. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Box<Option<bool>>,
    /// The directory path in the App Filesystem in which the tokens will be stored.
    #[builder(into, default)]
    #[serde(rename = "tokenStorePath")]
    pub r#token_store_path: Box<Option<String>>,
    /// The name of the app setting which contains the SAS URL of the blob storage containing the tokens.
    #[builder(into, default)]
    #[serde(rename = "tokenStoreSasSettingName")]
    pub r#token_store_sas_setting_name: Box<Option<String>>,
    /// Should the nonce be validated while completing the login flow. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "validateNonce")]
    pub r#validate_nonce: Box<Option<bool>>,
}
