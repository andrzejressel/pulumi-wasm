#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthorizerJwtConfiguration {
    /// List of the intended recipients of the JWT. A valid JWT must provide an aud that matches at least one entry in this list.
    #[builder(into, default)]
    #[serde(rename = "audiences")]
    pub r#audiences: Box<Option<Vec<String>>>,
    /// Base domain of the identity provider that issues JSON Web Tokens, such as the `endpoint` attribute of the `aws.cognito.UserPool` resource.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
}
