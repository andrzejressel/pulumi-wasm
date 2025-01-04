#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsFunctionAppAuthSettingsV2GithubV2 {
    /// The ID of the GitHub app used for login..
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The app setting name that contains the `client_secret` value used for GitHub Login.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<String>,
    /// The list of OAuth 2.0 scopes that should be requested as part of GitHub Login authentication.
    #[builder(into, default)]
    #[serde(rename = "loginScopes")]
    pub r#login_scopes: Box<Option<Vec<String>>>,
}
