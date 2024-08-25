#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessApplicationSaasApp {
    /// The URL where this applications tile redirects users.
    #[serde(rename = "appLauncherUrl")]
    pub r#app_launcher_url: Box<Option<String>>,
    #[serde(rename = "authType")]
    pub r#auth_type: Box<Option<String>>,
    /// The application client id.
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The application client secret, only returned on initial apply.
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// The service provider's endpoint that is responsible for receiving and parsing a SAML assertion.
    #[serde(rename = "consumerServiceUrl")]
    pub r#consumer_service_url: Box<Option<String>>,
    /// Custom attribute mapped from IDPs.
    #[serde(rename = "customAttributes")]
    pub r#custom_attributes:
        Box<Option<Vec<crate::types::AccessApplicationSaasAppCustomAttribute>>>,
    /// The relay state used if not provided by the identity provider.
    #[serde(rename = "defaultRelayState")]
    pub r#default_relay_state: Box<Option<String>>,
    /// The OIDC flows supported by this application.
    #[serde(rename = "grantTypes")]
    pub r#grant_types: Box<Option<Vec<String>>>,
    /// A regex to filter Cloudflare groups returned in ID token and userinfo endpoint.
    #[serde(rename = "groupFilterRegex")]
    pub r#group_filter_regex: Box<Option<String>>,
    /// The unique identifier for the SaaS application.
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: Box<Option<String>>,
    /// The format of the name identifier sent to the SaaS application.
    #[serde(rename = "nameIdFormat")]
    pub r#name_id_format: Box<Option<String>>,
    /// A [JSONata](https://jsonata.org/) expression that transforms an application's user identities into a NameID value for its SAML assertion. This expression should evaluate to a singular string. The output of this expression can override the `name_id_format` setting.
    #[serde(rename = "nameIdTransformJsonata")]
    pub r#name_id_transform_jsonata: Box<Option<String>>,
    /// The public certificate that will be used to verify identities.
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    /// The permitted URL's for Cloudflare to return Authorization codes and Access/ID tokens.
    #[serde(rename = "redirectUris")]
    pub r#redirect_uris: Box<Option<Vec<String>>>,
    /// A [JSONata](https://jsonata.org/) expression that transforms an application's user identities into attribute assertions in the SAML response. The expression can transform id, email, name, and groups values. It can also transform fields listed in the saml*attributes or oidc*fields of the identity provider used to authenticate. The output of this expression must be a JSON object.
    #[serde(rename = "samlAttributeTransformJsonata")]
    pub r#saml_attribute_transform_jsonata: Box<Option<String>>,
    /// Define the user information shared with access.
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    /// A globally unique name for an identity or service provider.
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Box<Option<String>>,
    /// The endpoint where the SaaS application will send login requests.
    #[serde(rename = "ssoEndpoint")]
    pub r#sso_endpoint: Box<Option<String>>,
}
