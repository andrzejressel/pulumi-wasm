#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialAuthToken {
    /// The token for the auth type.
    #[builder(into, default)]
    #[serde(rename = "token")]
    pub r#token: Box<Option<String>>,
    /// Authentication type, e.g. "Basic", "Bearer", etc.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
