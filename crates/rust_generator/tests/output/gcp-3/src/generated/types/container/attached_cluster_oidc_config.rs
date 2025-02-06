#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttachedClusterOidcConfig {
    /// A JSON Web Token (JWT) issuer URI. `issuer` must start with `https://`
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Box<String>,
    /// OIDC verification keys in JWKS format (RFC 7517).
    #[builder(into, default)]
    #[serde(rename = "jwks")]
    pub r#jwks: Box<Option<String>>,
}
