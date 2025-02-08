#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkloadIdentityPoolProviderOidc {
    /// Acceptable values for the `aud` field (audience) in the OIDC token. Token exchange
    /// requests are rejected if the token audience does not match one of the configured
    /// values. Each audience may be at most 256 characters. A maximum of 10 audiences may
    /// be configured.
    /// If this list is empty, the OIDC token audience must be equal to the full canonical
    /// resource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.
    /// For example:
    /// ```sh
    /// //iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>
    /// https://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>
    /// ```
    #[builder(into, default)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Box<Option<Vec<String>>>,
    /// The OIDC issuer URL.
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
}
