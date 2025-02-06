#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParams {
    /// A list of parameter map entries.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "entries")]
    pub r#entries: Box<Option<Vec<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntry>>>,
}
