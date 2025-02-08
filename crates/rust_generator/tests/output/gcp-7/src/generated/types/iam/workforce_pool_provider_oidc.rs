#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkforcePoolProviderOidc {
    /// The client ID. Must match the audience claim of the JWT issued by the identity provider.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The optional client secret. Required to enable Authorization Code flow for web sign-in.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<super::super::types::iam::WorkforcePoolProviderOidcClientSecret>>,
    /// The OIDC issuer URI. Must be a valid URI using the 'https' scheme.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Box<String>,
    /// OIDC JWKs in JSON String format. For details on definition of a
    /// JWK, see https:tools.ietf.org/html/rfc7517. If not set, then we
    /// use the `jwks_uri` from the discovery document fetched from the
    /// .well-known path for the `issuer_uri`. Currently, RSA and EC asymmetric
    /// keys are supported. The JWK must use following format and include only
    /// the following fields:
    /// ```sh
    /// {
    /// "keys": [
    /// {
    /// "kty": "RSA/EC",
    /// "alg": "<algorithm>",
    /// "use": "sig",
    /// "kid": "<key-id>",
    /// "n": "",
    /// "e": "",
    /// "x": "",
    /// "y": "",
    /// "crv": ""
    /// }
    /// ]
    /// }
    /// ```
    #[builder(into, default)]
    #[serde(rename = "jwksJson")]
    pub r#jwks_json: Box<Option<String>>,
    /// Configuration for web single sign-on for the OIDC provider. Here, web sign-in refers to console sign-in and gcloud sign-in through the browser.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "webSsoConfig")]
    pub r#web_sso_config: Box<Option<super::super::types::iam::WorkforcePoolProviderOidcWebSsoConfig>>,
}
