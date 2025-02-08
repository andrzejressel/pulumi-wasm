#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionAuthConfigOauth2JwtBearerJwtClaims {
    /// Value for the "aud" claim.
    /// 
    /// <a name="nested_oauth2_client_credentials"></a>The `oauth2_client_credentials` block supports:
    #[builder(into, default)]
    #[serde(rename = "audience")]
    pub r#audience: Box<Option<String>>,
    /// Value for the "iss" claim.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// Value for the "sub" claim.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
}
