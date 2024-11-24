#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationSaasAppHybridAndImplicitOptions {
    /// If true, the authorization endpoint will return an access token.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "returnAccessTokenFromAuthorizationEndpoint")]
    pub r#return_access_token_from_authorization_endpoint: Box<Option<bool>>,
    /// If true, the authorization endpoint will return an id token.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "returnIdTokenFromAuthorizationEndpoint")]
    pub r#return_id_token_from_authorization_endpoint: Box<Option<bool>>,
}
