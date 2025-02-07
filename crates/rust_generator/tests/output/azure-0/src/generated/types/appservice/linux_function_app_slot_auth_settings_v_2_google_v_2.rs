#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppSlotAuthSettingsV2GoogleV2 {
    /// Specifies a list of Allowed Audiences that should be requested as part of Google Sign-In authentication.
    #[builder(into, default)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Box<Option<Vec<String>>>,
    /// The OpenID Connect Client ID for the Google web application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The app setting name that contains the `client_secret` value used for Google Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<String>,
    /// The list of OAuth 2.0 scopes that should be requested as part of Google Sign-In authentication.
    #[builder(into, default)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Option<Vec<String>>>,
}
