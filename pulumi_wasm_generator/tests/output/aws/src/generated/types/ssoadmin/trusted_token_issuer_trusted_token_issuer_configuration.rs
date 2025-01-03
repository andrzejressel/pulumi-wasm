#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TrustedTokenIssuerTrustedTokenIssuerConfiguration {
    /// A block that describes the settings for a trusted token issuer that works with OpenID Connect (OIDC) by using JSON Web Tokens (JWT). See Documented below below.
    #[builder(into, default)]
    #[serde(rename = "oidcJwtConfiguration")]
    pub r#oidc_jwt_configuration: Box<Option<super::super::types::ssoadmin::TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration>>,
}
