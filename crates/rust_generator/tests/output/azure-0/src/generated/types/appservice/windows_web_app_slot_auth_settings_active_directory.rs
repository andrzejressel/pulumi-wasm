#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotAuthSettingsActiveDirectory {
    /// Specifies a list of Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    /// 
    /// > **Note:** The `client_id` value is always considered an allowed audience, so should not be included.
    #[builder(into, default)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Box<Option<Vec<String>>>,
    /// The ID of the Client to use to authenticate with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The Client Secret for the Client ID. Cannot be used with `client_secret_setting_name`.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// The App Setting name that contains the client secret of the Client. Cannot be used with `client_secret`.
    #[builder(into, default)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<Option<String>>,
}
