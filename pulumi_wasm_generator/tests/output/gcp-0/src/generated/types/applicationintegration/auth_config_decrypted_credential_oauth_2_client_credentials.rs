#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentials {
    /// The client's ID.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The client's secret.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// Represent how to pass parameters to fetch access token Possible values: ["REQUEST_TYPE_UNSPECIFIED", "REQUEST_BODY", "QUERY_PARAMETERS", "ENCODED_HEADER"]
    #[builder(into, default)]
    #[serde(rename = "requestType")]
    pub r#request_type: Box<Option<String>>,
    /// A space-delimited list of requested scope permissions.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// The token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token.
    #[builder(into, default)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<Option<String>>,
    /// Token parameters for the auth request.
    #[builder(into, default)]
    #[serde(rename = "tokenParams")]
    pub r#token_params: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParams>>,
}
