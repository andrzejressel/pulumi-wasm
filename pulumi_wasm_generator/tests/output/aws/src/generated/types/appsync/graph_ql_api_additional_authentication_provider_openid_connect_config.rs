#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GraphQlApiAdditionalAuthenticationProviderOpenidConnectConfig {
    /// Number of milliseconds a token is valid after being authenticated.
    #[builder(into, default)]
    #[serde(rename = "authTtl")]
    pub r#auth_ttl: Box<Option<i32>>,
    /// Client identifier of the Relying party at the OpenID identity provider. This identifier is typically obtained when the Relying party is registered with the OpenID identity provider. You can specify a regular expression so the AWS AppSync can validate against multiple client identifiers at a time.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// Number of milliseconds a token is valid after being issued to a user.
    #[builder(into, default)]
    #[serde(rename = "iatTtl")]
    pub r#iat_ttl: Box<Option<i32>>,
    /// Issuer for the OpenID Connect configuration. The issuer returned by discovery MUST exactly match the value of iss in the ID Token.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
}
