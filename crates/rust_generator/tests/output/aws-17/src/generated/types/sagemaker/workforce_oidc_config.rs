#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforceOidcConfig {
    /// A string to string map of identifiers specific to the custom identity provider (IdP) being used.
    #[builder(into, default)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Box<Option<std::collections::HashMap<String, String>>>,
    /// The OIDC IdP authorization endpoint used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: Box<String>,
    /// The OIDC IdP client ID used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The OIDC IdP client secret used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<String>,
    /// The OIDC IdP issuer used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// The OIDC IdP JSON Web Key Set (Jwks) URI used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "jwksUri")]
    pub r#jwks_uri: Box<String>,
    /// The OIDC IdP logout endpoint used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: Box<String>,
    /// An array of string identifiers used to refer to the specific pieces of user data or claims that the client application wants to access.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// The OIDC IdP token endpoint used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<String>,
    /// The OIDC IdP user information endpoint used to configure your private workforce.
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: Box<String>,
}
