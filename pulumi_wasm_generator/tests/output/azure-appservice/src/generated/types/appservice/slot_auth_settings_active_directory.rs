#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SlotAuthSettingsActiveDirectory {
    /// Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    #[builder(into, default)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Box<Option<Vec<String>>>,
    /// The Client ID of this relying party application. Enables OpenIDConnection authentication with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The Client Secret of this relying party application. If no secret is provided, implicit flow will be used.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
}