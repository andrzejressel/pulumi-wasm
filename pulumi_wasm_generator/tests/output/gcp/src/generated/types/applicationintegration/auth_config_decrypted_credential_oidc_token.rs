#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOidcToken {
    /// Audience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for.
    #[builder(into, default)]
    #[serde(rename = "audience")]
    pub r#audience: Box<Option<String>>,
    /// The service account email to be used as the identity for the token.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// (Output)
    /// ID token obtained for the service account.
    #[builder(into, default)]
    #[serde(rename = "token")]
    pub r#token: Box<Option<String>>,
    /// (Output)
    /// The approximate time until the token retrieved is valid.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "tokenExpireTime")]
    pub r#token_expire_time: Box<Option<String>>,
}
