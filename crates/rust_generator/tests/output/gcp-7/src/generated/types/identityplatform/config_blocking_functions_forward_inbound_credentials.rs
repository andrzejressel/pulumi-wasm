#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigBlockingFunctionsForwardInboundCredentials {
    /// Whether to pass the user's OAuth identity provider's access token.
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<bool>>,
    /// Whether to pass the user's OIDC identity provider's ID token.
    #[builder(into, default)]
    #[serde(rename = "idToken")]
    pub r#id_token: Box<Option<bool>>,
    /// Whether to pass the user's OAuth identity provider's refresh token.
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<bool>>,
}
