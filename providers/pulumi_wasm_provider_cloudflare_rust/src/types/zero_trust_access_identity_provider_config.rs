#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessIdentityProviderConfig {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "apiToken")]
    pub r#api_token: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "claims")]
    pub r#claims: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "directoryId")]
    pub r#directory_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Box<Option<String>>,
}
