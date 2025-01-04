#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotAuthSettingsV2AppleV2 {
    /// The OpenID Connect Client ID for the Apple web application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The app setting name that contains the `client_secret` value used for Apple Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<String>,
    /// A list of Login Scopes provided by this Authentication Provider.
    /// 
    /// > **NOTE:** This is configured on the Authentication Provider side and is Read Only here.
    #[builder(into, default)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Option<Vec<String>>>,
}
