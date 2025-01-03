#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SettingsAccessSettingsWorkforceIdentitySettingsOauth2 {
    /// The OAuth 2.0 client ID registered in the workforce identity
    /// federation OAuth 2.0 Server.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// Input only. The OAuth 2.0 client secret created while registering
    /// the client ID.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// Output only. SHA256 hash value for the client secret. This field
    /// is returned by IAP when the settings are retrieved.
    #[builder(into, default)]
    #[serde(rename = "clientSecretSha256")]
    pub r#client_secret_sha_256: Box<Option<String>>,
}
