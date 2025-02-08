#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionAuthConfigOauth2JwtBearer {
    /// Secret version reference containing a PKCS#8 PEM-encoded private key associated with the Client Certificate.
    /// This private key will be used to sign JWTs used for the jwt-bearer authorization grant.
    /// Specified in the form as: projects/*/secrets/*/versions/*.
    #[builder(into, default)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearerClientKey>>,
    /// JwtClaims providers fields to generate the token.
    #[builder(into, default)]
    #[serde(rename = "jwtClaims")]
    pub r#jwt_claims: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearerJwtClaims>>,
}
