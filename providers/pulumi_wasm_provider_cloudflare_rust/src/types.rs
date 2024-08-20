#[derive(serde::Serialize)]
pub struct AccessApplicationCorsHeader {
    /// Value to determine whether all HTTP headers are exposed.
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Box<Option<bool>>,
    /// Value to determine whether all methods are exposed.
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Box<Option<bool>>,
    /// Value to determine whether all origins are permitted to make CORS requests.
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Box<Option<bool>>,
    /// Value to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests.
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// List of HTTP headers to expose via CORS.
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    /// List of methods to expose via CORS.
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    /// List of origins permitted to make CORS requests.
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    /// The maximum time a preflight request will be cached.
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationFooterLink {
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The URL of the footer link.
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    /// The button text color of the landing page.
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    /// The URL of the image to be displayed in the landing page.
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    /// The message of the landing page.
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// The title of the landing page.
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}

#[derive(serde::Serialize)]
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

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttribute {
    /// A friendly name for the attribute as provided to the SaaS app.
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Box<Option<String>>,
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A globally unique name for an identity or service provider.
    #[serde(rename = "nameFormat")]
    pub r#name_format: Box<Option<String>>,
    /// True if the attribute must be always present.
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<crate::types::AccessApplicationSaasAppCustomAttributeSource>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttributeSource {
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessGroupExcludeAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessGroupExcludeAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessGroupExcludeExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessGroupExcludeGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessGroupExcludeGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessGroupExcludeOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessGroupExcludeSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupInclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessGroupIncludeAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessGroupIncludeAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessGroupIncludeExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessGroupIncludeGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessGroupIncludeGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessGroupIncludeOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessGroupIncludeSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequire {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessGroupRequireAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessGroupRequireAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessGroupRequireExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessGroupRequireGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessGroupRequireGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessGroupRequireOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessGroupRequireSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessIdentityProviderConfig {
    #[serde(rename = "apiToken")]
    pub r#api_token: Box<Option<String>>,
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Box<Option<String>>,
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Option<Vec<String>>>,
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Box<Option<String>>,
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Box<Option<String>>,
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Box<Option<String>>,
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Box<Option<String>>,
    #[serde(rename = "claims")]
    pub r#claims: Box<Option<Vec<String>>>,
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Box<Option<bool>>,
    #[serde(rename = "directoryId")]
    pub r#directory_id: Box<Option<String>>,
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Box<Option<String>>,
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Box<Option<String>>,
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Box<Option<String>>,
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Box<Option<String>>,
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Box<Option<String>>,
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Box<Option<String>>,
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Box<Option<String>>,
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Box<Option<bool>>,
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Box<Option<String>>,
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    #[serde(rename = "signRequest")]
    pub r#sign_request: Box<Option<bool>>,
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Box<Option<String>>,
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Box<Option<bool>>,
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessIdentityProviderScimConfig {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "groupMemberDeprovision")]
    pub r#group_member_deprovision: Box<Option<bool>>,
    #[serde(rename = "seatDeprovision")]
    pub r#seat_deprovision: Box<Option<bool>>,
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<String>>,
    #[serde(rename = "userDeprovision")]
    pub r#user_deprovision: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct AccessMutualTlsHostnameSettingsSetting {
    /// Request client certificates for this hostname in China. Can only be set to true if this zone is china network enabled.
    #[serde(rename = "chinaNetwork")]
    pub r#china_network: Box<Option<bool>>,
    /// Client Certificate Forwarding is a feature that takes the client cert provided by the eyeball to the edge, and forwards it to the origin as a HTTP header to allow logging on the origin.
    #[serde(rename = "clientCertificateForwarding")]
    pub r#client_certificate_forwarding: Box<Option<bool>>,
    /// The hostname that these settings apply to.
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationCustomPage {
    /// The id of the forbidden page.
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    /// The id of the identity denied page.
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// The text at the bottom of the login page.
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// The text at the top of the login page.
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// The URL of the logo on the login page.
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// The text color on the login page.
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyApprovalGroup {
    /// Number of approvals needed.
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<i32>,
    /// List of emails to request approval from.
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    #[serde(rename = "emailListUuid")]
    pub r#email_list_uuid: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessPolicyExcludeAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessPolicyExcludeAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessPolicyExcludeExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessPolicyExcludeGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessPolicyExcludeGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessPolicyExcludeOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessPolicyExcludeSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyInclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessPolicyIncludeAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessPolicyIncludeAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessPolicyIncludeExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessPolicyIncludeGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessPolicyIncludeGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessPolicyIncludeOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessPolicyIncludeSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequire {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessPolicyRequireAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessPolicyRequireAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessPolicyRequireExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessPolicyRequireGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessPolicyRequireGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessPolicyRequireOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessPolicyRequireSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireGithub {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessRuleConfiguration {
    /// The request property to target. Available values: `ip`, `ip6`, `ip_range`, `asn`, `country`. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The value to target. Depends on target's type. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AddressMapIp {
    /// An IPv4 or IPv6 address.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AddressMapMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[serde(rename = "canDelete")]
    pub r#can_delete: Box<Option<bool>>,
    /// Identifier of the account or zone.
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// The type of the membership.
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ApiShieldAuthIdCharacteristic {
    /// The name of the characteristic.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type of characteristic. Available values: `header`, `cookie`.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenCondition {
    /// Request IP related conditions.
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<crate::types::ApiTokenConditionRequestIp>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenConditionRequestIp {
    /// List of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses.
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// List of IP addresses or CIDR notation where the token should not be used from.
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenPolicy {
    /// Effect of the policy. Available values: `allow`, `deny`. Defaults to `allow`.
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// List of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information.
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Box<Vec<String>>,
    /// Describes what operations against which resources are allowed or denied.
    #[serde(rename = "resources")]
    pub r#resources: Box<std::collections::HashMap<String, String>>,
}

#[derive(serde::Serialize)]
pub struct CertificatePackValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CertificatePackValidationRecord {
    #[serde(rename = "cnameName")]
    pub r#cname_name: Box<Option<String>>,
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Box<Option<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "httpBody")]
    pub r#http_body: Box<Option<String>>,
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    #[serde(rename = "txtName")]
    pub r#txt_name: Box<Option<String>>,
    #[serde(rename = "txtValue")]
    pub r#txt_value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSsl {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    /// If a custom uploaded certificate is used.
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Box<Option<String>>,
    /// The key for a custom uploaded certificate.
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<String>>,
    /// Domain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`.
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// SSL/TLS settings for the certificate.
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<crate::types::CustomHostnameSslSetting>>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Level of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Box<Option<Vec<crate::types::CustomHostnameSslValidationError>>>,
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Box<Option<Vec<crate::types::CustomHostnameSslValidationRecord>>>,
    /// Indicates whether the certificate covers a wildcard.
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslSetting {
    /// List of SSL/TLS ciphers to associate with this certificate.
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    /// Whether early hints should be supported. Available values: `on`, `off`.
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    /// Whether HTTP2 should be supported. Available values: `on`, `off`.
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    /// Lowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`.
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    /// Whether TLSv1.3 should be supported. Available values: `on`, `off`.
    #[serde(rename = "tls13")]
    pub r#tls_13: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslValidationRecord {
    #[serde(rename = "cnameName")]
    pub r#cname_name: Box<Option<String>>,
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Box<Option<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "httpBody")]
    pub r#http_body: Box<Option<String>>,
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    #[serde(rename = "txtName")]
    pub r#txt_name: Box<Option<String>>,
    #[serde(rename = "txtValue")]
    pub r#txt_value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomSslCustomSslOptions {
    /// Method of building intermediate certificate chain. A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    /// Certificate certificate and the intermediate(s).
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    /// Specifies the region where your private key can be held locally. Available values: `us`, `eu`, `highest_security`.
    #[serde(rename = "geoRestrictions")]
    pub r#geo_restrictions: Box<Option<String>>,
    /// Certificate's private key.
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    /// Whether to enable support for legacy clients which do not include SNI in the TLS handshake. Available values: `legacy_custom`, `sni_custom`.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomSslCustomSslPriority {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct DeviceDexTestData {
    /// The host URL for `http` test `kind`. For `traceroute`, it must be a valid hostname or IP address.
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The type of Device Dex Test. Available values: `http`, `traceroute`.
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    /// The http request method. Available values: `GET`.
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DeviceManagedNetworksConfig {
    /// The SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate.
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<String>,
    /// A network address of the form "host:port" that the WARP client will use to detect the presence of a TLS host.
    #[serde(rename = "tlsSockaddr")]
    pub r#tls_sockaddr: Box<String>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureIntegrationConfig {
    /// The Access client ID to be used as the `Cf-Access-Client-ID` header when making a request to the `api_url`.
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Box<Option<String>>,
    /// The Access client secret to be used as the `Cf-Access-Client-Secret` header when making a request to the `api_url`.
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Box<Option<String>>,
    /// The third-party API's URL.
    #[serde(rename = "apiUrl")]
    pub r#api_url: Box<Option<String>>,
    /// The third-party authorization API URL.
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    /// The client identifier for authenticating API calls.
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The client key for authenticating API calls.
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// The client secret for authenticating API calls.
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// The customer identifier for authenticating API calls.
    #[serde(rename = "customerId")]
    pub r#customer_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleInput {
    /// The number of active threats from SentinelOne.
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Box<Option<i32>>,
    /// The UUID of a Cloudflare managed certificate.
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    /// Specific volume(s) to check for encryption.
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Box<Option<Vec<String>>>,
    /// The common name for a certificate.
    #[serde(rename = "cn")]
    pub r#cn: Box<Option<String>>,
    /// The workspace one device compliance status. Available values: `compliant`, `noncompliant`.
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<Option<String>>,
    /// The workspace one connection id.
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    /// The count comparison operator for kolide. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "countOperator")]
    pub r#count_operator: Box<Option<String>>,
    /// The domain that the client must join.
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// The datetime a device last seen in RFC 3339 format from Tanium.
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Box<Option<String>>,
    /// True if the firewall must be enabled.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Checks if the file should exist.
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<bool>>,
    /// The Teams List id. Required for `serial_number` and `unique_client_id` rule types.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// True if SentinelOne device is infected.
    #[serde(rename = "infected")]
    pub r#infected: Box<Option<bool>>,
    /// True if SentinelOne device is active.
    #[serde(rename = "isActive")]
    pub r#is_active: Box<Option<bool>>,
    /// The number of issues for kolide.
    #[serde(rename = "issueCount")]
    pub r#issue_count: Box<Option<String>>,
    /// The duration of time that the host was last seen from Crowdstrike. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The network status from SentinelOne. Available values: `connected`, `disconnected`, `disconnecting`, `connecting`.
    #[serde(rename = "networkStatus")]
    pub r#network_status: Box<Option<String>>,
    /// The version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// OS signal score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "os")]
    pub r#os: Box<Option<String>>,
    /// The operating system excluding version information.
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The operating system version excluding OS name information or release name.
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// Overall ZTA score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "overall")]
    pub r#overall: Box<Option<String>>,
    /// The path to the file.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// True if all drives must be encrypted.
    #[serde(rename = "requireAll")]
    pub r#require_all: Box<Option<bool>>,
    /// The risk level from Tanium. Available values: `low`, `medium`, `high`, `critical`.
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<Option<String>>,
    /// Checks if the application should be running.
    #[serde(rename = "running")]
    pub r#running: Box<Option<bool>>,
    /// Sensor signal score from Crowdstrike. Value must be between 1 and 100.
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Box<Option<String>>,
    /// The sha256 hash of the file.
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
    /// The host’s current online status from Crowdstrike. Available values: `online`, `offline`, `unknown`.
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The thumbprint of the file certificate.
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
    /// The total score from Tanium.
    #[serde(rename = "totalScore")]
    pub r#total_score: Box<Option<i32>>,
    /// The operating system semantic version.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// The version comparison operator for crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleMatch {
    /// The platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`.
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwareness {
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Content types to exclude from context analysis and return all matches.
    #[serde(rename = "skip")]
    pub r#skip: Box<crate::types::DlpProfileContextAwarenessSkip>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwarenessSkip {
    /// Return all matches, regardless of context analysis result, if the data is a file.
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntry {
    /// Whether the entry is active. Defaults to `false`.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique entry identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the entry to deploy.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<crate::types::DlpProfileEntryPattern>>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    /// The validation algorithm to apply with this pattern.
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllAction {
    /// Type of supported action. Available values: `drop`, `forward`, `worker`.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// A list with items in the following form.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllMatcher {
    /// Type of matcher. Available values: `all`.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleAction {
    /// Type of action. Available values: `forward`, `worker`, `drop`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleMatcher {
    /// Field to match on. Required for `type` of `literal`.
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// Type of matcher. Available values: `literal`, `all`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct FallbackDomainDomain {
    /// A description of the fallback domain, displayed in the client UI.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of IP addresses to handle domain resolution.
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// The domain suffix to match when resolving locally.
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct HealthcheckHeader {
    /// The header name.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// A list of string values for the header.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigOrigin {
    /// The name of your origin database.
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The host (hostname or IP) of your origin database.
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The password of the Hyperdrive configuration.
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The port (default: 5432 for Postgres) of your origin database.
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Specifies the URL scheme used to connect to your origin database.
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
    /// The user of your origin database.
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItem {
    /// An optional comment for the item.
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<crate::types::ListItemValue>,
}

#[derive(serde::Serialize)]
pub struct ListItemHostname {
    /// The FQDN to match on.
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItemRedirect {
    /// Whether the redirect also matches subdomains of the source url.
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<bool>>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// The source url of the redirect.
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    /// The status code to be used when redirecting a request.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Whether the redirect also matches subpaths of the source url.
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<bool>>,
    /// The target url of the redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItemValue {
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Option<Vec<crate::types::ListItemValueHostname>>>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Option<Vec<crate::types::ListItemValueRedirect>>>,
}

#[derive(serde::Serialize)]
pub struct ListItemValueHostname {
    /// The FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com.
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItemValueRedirect {
    /// Whether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`.
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<String>>,
    /// Whether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`.
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<String>>,
    /// Whether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<String>>,
    /// The source url of the redirect.
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    /// The status code to be used when redirecting a request.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Whether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`.
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<String>>,
    /// The target url of the redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerAdaptiveRouting {
    /// Extends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`.
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerCountryPool {
    /// A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country.
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    /// A list of pool IDs in failover priority to use in the given country.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerLocationStrategy {
    /// Determines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Whether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when `steering_policy="proximity"`, and `geo` will prefer ECS only when `steering_policy="geo"`. Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`.
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerMonitorHeader {
    /// The header name.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// A list of values for the header.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100. Defaults to `0`.
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`. Defaults to `""`.
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    /// Percent of session traffic to shed 0 - 100. Defaults to `0`.
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`. Defaults to `""`.
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOrigin {
    /// The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname.
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Whether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// HTTP request headers.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::LoadBalancerPoolOriginHeader>>>,
    /// A human-identifiable name for the origin.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. When `origin_steering.policy="least_outstanding_requests"`, weight is used to scale the origin's outstanding requests. When `origin_steering.policy="least_connections"`, weight is used to scale the origin's open connections. Defaults to `1`.
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginHeader {
    /// HTTP request headers.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginSteering {
    /// Origin steering policy to be used. Value `random` selects an origin randomly. Value `hash` selects an origin by computing a hash over the CF-Connecting-IP address. Value `least_outstanding_requests` selects an origin by taking into consideration origin weights, as well as each origin's number of outstanding requests. Origins with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects an origin by taking into consideration origin weights, as well as each origin's number of open connections. Origins with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Available values: `""`, `hash`, `random`, `least_outstanding_requests`, `least_connections`. Defaults to `random`.
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPopPool {
    /// A list of pool IDs in failover priority to use for traffic reaching the given PoP.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP.
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRandomSteering {
    /// The default weight for pools in the load balancer that are not specified in the `pool_weights` map.
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    /// A mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer.
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRegionPool {
    /// A list of pool IDs in failover priority to use in the given region.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region.
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRule {
    /// The statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules).
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// A disabled rule will not be executed.
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Settings for a HTTP response to return directly to the eyeball if the condition is true. Note: `overrides` or `fixed_response` must be set.
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<crate::types::LoadBalancerRuleFixedResponse>>,
    /// Human readable name for this rule.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The load balancer settings to alter if this rule's `condition` is true. Note: `overrides` or `fixed_response` must be set.
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<crate::types::LoadBalancerRuleOverride>>>,
    /// Priority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used.
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// Terminates indicates that if this rule is true no further rules should be executed. Note: setting a `fixed_response` forces this field to `true`.
    #[serde(rename = "terminates")]
    pub r#terminates: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleFixedResponse {
    /// The value of the HTTP context-type header for this fixed response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// The value of the HTTP location header for this fixed response.
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The text used as the html body for this fixed response.
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// The HTTP status code used for this fixed response.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverride {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    #[serde(rename = "countryPools")]
    pub r#country_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideCountryPool>>>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Box<Option<Vec<String>>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Box<Option<String>>,
    /// Controls location-based steering for non-proxied requests.
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideLocationStrategy>>>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    #[serde(rename = "popPools")]
    pub r#pop_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverridePopPool>>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    #[serde(rename = "regionPools")]
    pub r#region_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRegionPool>>>,
    /// Configure attributes for session affinity.
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<String>>,
    /// Configure attributes for session affinity. Note that the property `drain_duration` is not currently supported as a rule override.
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Box<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Box<Option<String>>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`.
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideAdaptiveRouting {
    /// Extends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`.
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideCountryPool {
    /// A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country.
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    /// A list of pool IDs in failover priority to use in the given country.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideLocationStrategy {
    /// Determines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Whether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when `steering_policy="proximity"`, and `geo` will prefer ECS only when `steering_policy="geo"`. Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`.
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverridePopPool {
    /// A list of pool IDs in failover priority to use for traffic reaching the given PoP.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP.
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRandomSteering {
    /// The default weight for pools in the load balancer that are not specified in the `pool_weights` map.
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    /// A mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer.
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRegionPool {
    /// A list of pool IDs in failover priority to use in the given region.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region.
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideSessionAffinityAttribute {
    /// Configures the HTTP header names to use when header session affinity is enabled.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    /// Configures how headers are used when header session affinity is enabled. Set to true to require all headers to be present on requests in order for sessions to be created or false to require at least one header to be present. Defaults to `false`.
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    /// Configures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set `secure="Never"`. Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`.
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    /// Configures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`.
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    /// Configures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`.
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerSessionAffinityAttribute {
    /// Configures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer. Defaults to `0`.
    #[serde(rename = "drainDuration")]
    pub r#drain_duration: Box<Option<i32>>,
    /// Configures the HTTP header names to use when header session affinity is enabled.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    /// Configures how headers are used when header session affinity is enabled. Set to true to require all headers to be present on requests in order for sessions to be created or false to require at least one header to be present. Defaults to `false`.
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    /// Configures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set `secure="Never"`. Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`.
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    /// Configures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`.
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    /// Configures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`.
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Box<Option<String>>,
    /// String to be appended after each batch.
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Box<Option<String>>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Box<Option<bool>>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// List of field names to be included in the Logpush output.
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    /// String to be inserted in-between the records as separator.
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Box<Option<String>>,
    /// String to be prepended before each record. Defaults to `{`.
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Box<Option<String>>,
    /// String to be appended after each record. Defaults to `}`.
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Box<Option<String>>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Box<Option<String>>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedRequestHeader {
    /// Whether the headers rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Unique headers rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedResponseHeader {
    /// Whether the headers rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Unique headers rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyEmailIntegration {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyFilters {
    /// Targeted actions for alert.
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    /// Affected components for alert. Available values: `API`, `API Shield`, `Access`, `Always Online`, `Analytics`, `Apps Marketplace`, `Argo Smart Routing`, `Audit Logs`, `Authoritative DNS`, `Billing`, `Bot Management`, `Bring Your Own IP (BYOIP)`, `Browser Isolation`, `CDN Cache Purge`, `CDN/Cache`, `Cache Reserve`, `Challenge Platform`, `Cloud Access Security Broker (CASB)`, `Community Site`, `DNS Root Servers`, `DNS Updates`, `Dashboard`, `Data Loss Prevention (DLP)`, `Developer's Site`, `Digital Experience Monitoring (DEX)`, `Distributed Web Gateway`, `Durable Objects`, `Email Routing`, `Ethereum Gateway`, `Firewall`, `Gateway`, `Geo-Key Manager`, `Image Resizing`, `Images`, `Infrastructure`, `Lists`, `Load Balancing and Monitoring`, `Logs`, `Magic Firewall`, `Magic Transit`, `Magic WAN`, `Magic WAN Connector`, `Marketing Site`, `Mirage`, `Network`, `Notifications`, `Observatory`, `Page Shield`, `Pages`, `R2`, `Radar`, `Randomness Beacon`, `Recursive DNS`, `Registrar`, `Registration Data Access Protocol (RDAP)`, `SSL Certificate Provisioning`, `SSL for SaaS Provisioning`, `Security Center`, `Snippets`, `Spectrum`, `Speed Optimizations`, `Stream`, `Support Site`, `Time Services`, `Trace`, `Tunnel`, `Turnstile`, `WARP`, `Waiting Room`, `Web Analytics`, `Workers`, `Workers KV`, `Workers Preview`, `Zaraz`, `Zero Trust`, `Zero Trust Dashboard`, `Zone Versioning`.
    #[serde(rename = "affectedComponents")]
    pub r#affected_components: Box<Option<Vec<String>>>,
    /// Filter on Points of Presence.
    #[serde(rename = "airportCodes")]
    pub r#airport_codes: Box<Option<Vec<String>>>,
    /// Alert trigger preferences. Example: `slo`.
    #[serde(rename = "alertTriggerPreferences")]
    pub r#alert_trigger_preferences: Box<Option<Vec<String>>>,
    /// State of the pool to alert on.
    #[serde(rename = "enableds")]
    pub r#enableds: Box<Option<Vec<String>>>,
    /// Environment of pages. Available values: `ENVIRONMENT_PREVIEW`, `ENVIRONMENT_PRODUCTION`.
    #[serde(rename = "environments")]
    pub r#environments: Box<Option<Vec<String>>>,
    /// Source configuration to alert on for pool or origin.
    #[serde(rename = "eventSources")]
    pub r#event_sources: Box<Option<Vec<String>>>,
    /// Stream event type to alert on.
    #[serde(rename = "eventTypes")]
    pub r#event_types: Box<Option<Vec<String>>>,
    /// Pages event to alert. Available values: `EVENT_DEPLOYMENT_STARTED`, `EVENT_DEPLOYMENT_FAILED`, `EVENT_DEPLOYMENT_SUCCESS`.
    #[serde(rename = "events")]
    pub r#events: Box<Option<Vec<String>>>,
    /// Alert grouping.
    #[serde(rename = "groupBies")]
    pub r#group_bies: Box<Option<Vec<String>>>,
    /// Identifier health check. Required when using `filters.0.status`.
    #[serde(rename = "healthCheckIds")]
    pub r#health_check_ids: Box<Option<Vec<String>>>,
    /// The incident impact level that will trigger the dispatch of a notification. Available values: `INCIDENT_IMPACT_NONE`, `INCIDENT_IMPACT_MINOR`, `INCIDENT_IMPACT_MAJOR`, `INCIDENT_IMPACT_CRITICAL`.
    #[serde(rename = "incidentImpacts")]
    pub r#incident_impacts: Box<Option<Vec<String>>>,
    /// Stream input id to alert on.
    #[serde(rename = "inputIds")]
    pub r#input_ids: Box<Option<Vec<String>>>,
    /// A numerical limit. Example: `100`.
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<Vec<String>>>,
    /// Megabits per second threshold for dos alert.
    #[serde(rename = "megabitsPerSeconds")]
    pub r#megabits_per_seconds: Box<Option<Vec<String>>>,
    /// Health status to alert on for pool or origin.
    #[serde(rename = "newHealths")]
    pub r#new_healths: Box<Option<Vec<String>>>,
    /// Tunnel health status to alert on.
    #[serde(rename = "newStatuses")]
    pub r#new_statuses: Box<Option<Vec<String>>>,
    /// Packets per second threshold for dos alert.
    #[serde(rename = "packetsPerSeconds")]
    pub r#packets_per_seconds: Box<Option<Vec<String>>>,
    /// Load balancer pool identifier.
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Option<Vec<String>>>,
    /// Product name. Available values: `worker_requests`, `worker_durable_objects_requests`, `worker_durable_objects_duration`, `worker_durable_objects_data_transfer`, `worker_durable_objects_stored_data`, `worker_durable_objects_storage_deletes`, `worker_durable_objects_storage_writes`, `worker_durable_objects_storage_reads`.
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Identifier of pages project.
    #[serde(rename = "projectIds")]
    pub r#project_ids: Box<Option<Vec<String>>>,
    /// Protocol to alert on for dos.
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<String>>>,
    /// Requests per second threshold for dos alert.
    #[serde(rename = "requestsPerSeconds")]
    pub r#requests_per_seconds: Box<Option<Vec<String>>>,
    /// Selectors for alert. Valid options depend on the alert type.
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Option<Vec<String>>>,
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<String>>>,
    /// A numerical limit. Example: `99.9`.
    #[serde(rename = "slos")]
    pub r#slos: Box<Option<Vec<String>>>,
    /// Status to alert on.
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<String>>>,
    /// Target host to alert on for dos.
    #[serde(rename = "targetHostnames")]
    pub r#target_hostnames: Box<Option<Vec<String>>>,
    /// Target domain to alert on.
    #[serde(rename = "targetZoneNames")]
    pub r#target_zone_names: Box<Option<Vec<String>>>,
    /// Tunnel IDs to alert on.
    #[serde(rename = "tunnelIds")]
    pub r#tunnel_ids: Box<Option<Vec<String>>>,
    /// Filter for alert.
    #[serde(rename = "wheres")]
    pub r#wheres: Box<Option<Vec<String>>>,
    /// A list of zone identifiers.
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyPagerdutyIntegration {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyWebhooksIntegration {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActions {
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<bool>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    /// The Time To Live for the browser cache. `0` means 'Respect Existing Headers'
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    /// String value of cookie name to conditionally bypass cache the page.
    #[serde(rename = "bypassCacheOnCookie")]
    pub r#bypass_cache_on_cookie: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<String>>,
    /// Controls how Cloudflare creates Cache Keys used to identify files in cache. See below for full description.
    #[serde(rename = "cacheKeyFields")]
    pub r#cache_key_fields: Box<Option<crate::types::PageRuleActionsCacheKeyFields>>,
    /// Whether to set the cache level to `"bypass"`, `"basic"`, `"simplified"`, `"aggressive"`, or `"cache_everything"`.
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    /// String value of cookie name to conditionally cache the page.
    #[serde(rename = "cacheOnCookie")]
    pub r#cache_on_cookie: Box<Option<String>>,
    /// Set cache TTL based on the response status from the origin web server. Can be specified multiple times. See below for full description.
    #[serde(rename = "cacheTtlByStatuses")]
    pub r#cache_ttl_by_statuses: Box<Option<Vec<crate::types::PageRuleActionsCacheTtlByStatus>>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "disablePerformance")]
    pub r#disable_performance: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "disableSecurity")]
    pub r#disable_security: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// The Time To Live for the edge cache.
    #[serde(rename = "edgeCacheTtl")]
    pub r#edge_cache_ttl: Box<Option<i32>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    /// Whether origin Cache-Control action is `"on"` or `"off"`.
    #[serde(rename = "explicitCacheControl")]
    pub r#explicit_cache_control: Box<Option<String>>,
    /// The URL to forward to, and with what status. See below.
    #[serde(rename = "forwardingUrl")]
    pub r#forwarding_url: Box<Option<crate::types::PageRuleActionsForwardingUrl>>,
    /// Value of the Host header to send.
    #[serde(rename = "hostHeaderOverride")]
    pub r#host_header_override: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    /// The configuration for HTML, CSS and JS minification. See below for full list of options.
    #[serde(rename = "minifies")]
    pub r#minifies: Box<Option<Vec<crate::types::PageRuleActionsMinify>>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    /// Whether this action is `"off"`, `"lossless"` or `"lossy"`.
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Overridden origin server name.
    #[serde(rename = "resolveOverride")]
    pub r#resolve_override: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "respectStrongEtag")]
    pub r#respect_strong_etag: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    /// Whether to set the rocket loader to `"on"`, `"off"`.
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    /// Whether to set the security level to `"off"`, `"essentially_off"`, `"low"`, `"medium"`, `"high"`, or `"under_attack"`.
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    /// Whether to set the SSL mode to `"off"`, `"flexible"`, `"full"`, `"strict"`, or `"origin_pull"`.
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFields {
    /// Controls what cookies go into Cache Key:
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>>,
    /// Controls what HTTP headers go into Cache Key:
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>>,
    /// Controls which Host header goes into Cache Key:
    #[serde(rename = "host")]
    pub r#host: Box<crate::types::PageRuleActionsCacheKeyFieldsHost>,
    /// Controls which URL query string parameters go into the Cache Key.
    #[serde(rename = "queryString")]
    pub r#query_string: Box<crate::types::PageRuleActionsCacheKeyFieldsQueryString>,
    /// Controls which end user-related features go into the Cache Key.
    #[serde(rename = "user")]
    pub r#user: Box<crate::types::PageRuleActionsCacheKeyFieldsUser>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    /// Check for presence of specified HTTP headers, without including their actual values.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHeader {
    /// Check for presence of specified HTTP headers, without including their actual values.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Exclude these query string parameters from Cache Key.
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHost {
    /// `false` (default) - includes the Host header in the HTTP request sent to the origin; `true` - includes the Host header that was resolved to get the origin IP for the request (e.g. changed with Resolve Override Page Rule).
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    /// Exclude these query string parameters from Cache Key.
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// `false` (default) - all query string parameters are used for Cache Key, unless explicitly excluded; `true` - all query string parameters are ignored; value should be `false` if any of `exclude` or `include` is non-empty.
    #[serde(rename = "ignore")]
    pub r#ignore: Box<Option<bool>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    /// `true` - classifies a request as “mobile”, “desktop”, or “tablet” based on the User Agent; defaults to `false`.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// `true` - includes the client’s country, derived from the IP address; defaults to `false`.
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// `true` - includes the first language code contained in the `Accept-Language` header sent by the client; defaults to `false`.
    ///
    /// Example:
    ///
    /// <!--Start PulumiCodeChooser -->
    /// ### Typescript
    /// ```typescript
    /// import * as pulumi from "@pulumi/pulumi";
    /// import * as cloudflare from "@pulumi/cloudflare";
    ///
    /// // Unrealistic example with all features used
    /// const foobar = new cloudflare.PageRule("foobar", {
    ///     zoneId: _var.cloudflare_zone_id,
    ///     target: `${_var.cloudflare_zone}/app/*`,
    ///     priority: 1,
    ///     actions: {
    ///         cacheKeyFields: {
    ///             cookie: {
    ///                 checkPresences: ["wordpress_test_cookie"],
    ///             },
    ///             header: {
    ///                 checkPresences: ["header_present"],
    ///                 excludes: ["origin"],
    ///                 includes: [
    ///                     "api-key",
    ///                     "dnt",
    ///                 ],
    ///             },
    ///             host: {
    ///                 resolved: true,
    ///             },
    ///             queryString: {
    ///                 ignore: true,
    ///             },
    ///             user: {
    ///                 deviceType: false,
    ///                 geo: true,
    ///                 lang: true,
    ///             },
    ///         },
    ///     },
    /// });
    /// ```
    /// ### Python
    /// ```python
    /// import pulumi
    /// import pulumi_cloudflare as cloudflare
    ///
    /// # Unrealistic example with all features used
    /// foobar = cloudflare.PageRule("foobar",
    ///     zone_id=var["cloudflare_zone_id"],
    ///     target=f"{var['cloudflare_zone']}/app/*",
    ///     priority=1,
    ///     actions=cloudflare.PageRuleActionsArgs(
    ///         cache_key_fields=cloudflare.PageRuleActionsCacheKeyFieldsArgs(
    ///             cookie=cloudflare.PageRuleActionsCacheKeyFieldsCookieArgs(
    ///                 check_presences=["wordpress_test_cookie"],
    ///             ),
    ///             header=cloudflare.PageRuleActionsCacheKeyFieldsHeaderArgs(
    ///                 check_presences=["header_present"],
    ///                 excludes=["origin"],
    ///                 includes=[
    ///                     "api-key",
    ///                     "dnt",
    ///                 ],
    ///             ),
    ///             host=cloudflare.PageRuleActionsCacheKeyFieldsHostArgs(
    ///                 resolved=True,
    ///             ),
    ///             query_string=cloudflare.PageRuleActionsCacheKeyFieldsQueryStringArgs(
    ///                 ignore=True,
    ///             ),
    ///             user=cloudflare.PageRuleActionsCacheKeyFieldsUserArgs(
    ///                 device_type=False,
    ///                 geo=True,
    ///                 lang=True,
    ///             ),
    ///         ),
    ///     ))
    /// ```
    /// ### C#
    /// ```csharp
    /// using System.Collections.Generic;
    /// using System.Linq;
    /// using Pulumi;
    /// using Cloudflare = Pulumi.Cloudflare;
    ///
    /// return await Deployment.RunAsync(() =>
    /// {
    ///     // Unrealistic example with all features used
    ///     var foobar = new Cloudflare.PageRule("foobar", new()
    ///     {
    ///         ZoneId = @var.Cloudflare_zone_id,
    ///         Target = $"{@var.Cloudflare_zone}/app/*",
    ///         Priority = 1,
    ///         Actions = new Cloudflare.Inputs.PageRuleActionsArgs
    ///         {
    ///             CacheKeyFields = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsArgs
    ///             {
    ///                 Cookie = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsCookieArgs
    ///                 {
    ///                     CheckPresences = new[]
    ///                     {
    ///                         "wordpress_test_cookie",
    ///                     },
    ///                 },
    ///                 Header = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsHeaderArgs
    ///                 {
    ///                     CheckPresences = new[]
    ///                     {
    ///                         "header_present",
    ///                     },
    ///                     Excludes = new[]
    ///                     {
    ///                         "origin",
    ///                     },
    ///                     Includes = new[]
    ///                     {
    ///                         "api-key",
    ///                         "dnt",
    ///                     },
    ///                 },
    ///                 Host = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsHostArgs
    ///                 {
    ///                     Resolved = true,
    ///                 },
    ///                 QueryString = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsQueryStringArgs
    ///                 {
    ///                     Ignore = true,
    ///                 },
    ///                 User = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsUserArgs
    ///                 {
    ///                     DeviceType = false,
    ///                     Geo = true,
    ///                     Lang = true,
    ///                 },
    ///             },
    ///         },
    ///     });
    ///
    /// });
    /// ```
    /// ### Go
    /// ```go
    /// package main
    ///
    /// import (
    /// 	"fmt"
    ///
    /// 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
    /// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
    /// )
    ///
    /// func main() {
    /// 	pulumi.Run(func(ctx *pulumi.Context) error {
    /// 		// Unrealistic example with all features used
    /// 		_, err := cloudflare.NewPageRule(ctx, "foobar", &cloudflare.PageRuleArgs{
    /// 			ZoneId:   pulumi.Any(_var.Cloudflare_zone_id),
    /// 			Target:   pulumi.String(fmt.Sprintf("%v/app/*", _var.Cloudflare_zone)),
    /// 			Priority: pulumi.Int(1),
    /// 			Actions: &cloudflare.PageRuleActionsArgs{
    /// 				CacheKeyFields: &cloudflare.PageRuleActionsCacheKeyFieldsArgs{
    /// 					Cookie: &cloudflare.PageRuleActionsCacheKeyFieldsCookieArgs{
    /// 						CheckPresences: pulumi.StringArray{
    /// 							pulumi.String("wordpress_test_cookie"),
    /// 						},
    /// 					},
    /// 					Header: &cloudflare.PageRuleActionsCacheKeyFieldsHeaderArgs{
    /// 						CheckPresences: pulumi.StringArray{
    /// 							pulumi.String("header_present"),
    /// 						},
    /// 						Excludes: pulumi.StringArray{
    /// 							pulumi.String("origin"),
    /// 						},
    /// 						Includes: pulumi.StringArray{
    /// 							pulumi.String("api-key"),
    /// 							pulumi.String("dnt"),
    /// 						},
    /// 					},
    /// 					Host: &cloudflare.PageRuleActionsCacheKeyFieldsHostArgs{
    /// 						Resolved: pulumi.Bool(true),
    /// 					},
    /// 					QueryString: &cloudflare.PageRuleActionsCacheKeyFieldsQueryStringArgs{
    /// 						Ignore: pulumi.Bool(true),
    /// 					},
    /// 					User: &cloudflare.PageRuleActionsCacheKeyFieldsUserArgs{
    /// 						DeviceType: pulumi.Bool(false),
    /// 						Geo:        pulumi.Bool(true),
    /// 						Lang:       pulumi.Bool(true),
    /// 					},
    /// 				},
    /// 			},
    /// 		})
    /// 		if err != nil {
    /// 			return err
    /// 		}
    /// 		return nil
    /// 	})
    /// }
    /// ```
    /// ### Java
    /// ```java
    /// package generated_program;
    ///
    /// import com.pulumi.Context;
    /// import com.pulumi.Pulumi;
    /// import com.pulumi.core.Output;
    /// import com.pulumi.cloudflare.PageRule;
    /// import com.pulumi.cloudflare.PageRuleArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsCookieArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsHeaderArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsHostArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsQueryStringArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsUserArgs;
    /// import java.util.List;
    /// import java.util.ArrayList;
    /// import java.util.Map;
    /// import java.io.File;
    /// import java.nio.file.Files;
    /// import java.nio.file.Paths;
    ///
    /// public class App {
    ///     public static void main(String[] args) {
    ///         Pulumi.run(App::stack);
    ///     }
    ///
    ///     public static void stack(Context ctx) {
    ///         // Unrealistic example with all features used
    ///         var foobar = new PageRule("foobar", PageRuleArgs.builder()        
    ///             .zoneId(var_.cloudflare_zone_id())
    ///             .target(String.format("%s/app/*", var_.cloudflare_zone()))
    ///             .priority(1)
    ///             .actions(PageRuleActionsArgs.builder()
    ///                 .cacheKeyFields(PageRuleActionsCacheKeyFieldsArgs.builder()
    ///                     .cookie(PageRuleActionsCacheKeyFieldsCookieArgs.builder()
    ///                         .checkPresences("wordpress_test_cookie")
    ///                         .build())
    ///                     .header(PageRuleActionsCacheKeyFieldsHeaderArgs.builder()
    ///                         .checkPresences("header_present")
    ///                         .excludes("origin")
    ///                         .includes(                        
    ///                             "api-key",
    ///                             "dnt")
    ///                         .build())
    ///                     .host(PageRuleActionsCacheKeyFieldsHostArgs.builder()
    ///                         .resolved(true)
    ///                         .build())
    ///                     .queryString(PageRuleActionsCacheKeyFieldsQueryStringArgs.builder()
    ///                         .ignore(true)
    ///                         .build())
    ///                     .user(PageRuleActionsCacheKeyFieldsUserArgs.builder()
    ///                         .deviceType(false)
    ///                         .geo(true)
    ///                         .lang(true)
    ///                         .build())
    ///                     .build())
    ///                 .build())
    ///             .build());
    ///
    ///     }
    /// }
    /// ```
    /// ### YAML
    /// ```yaml
    /// resources:
    ///   # Unrealistic example with all features used
    ///   foobar:
    ///     type: cloudflare:PageRule
    ///     properties:
    ///       zoneId: ${var.cloudflare_zone_id}
    ///       target: ${var.cloudflare_zone}/app/*
    ///       priority: 1
    ///       actions:
    ///         cacheKeyFields:
    ///           cookie:
    ///             checkPresences:
    ///               - wordpress_test_cookie
    ///           header:
    ///             checkPresences:
    ///               - header_present
    ///             excludes:
    ///               - origin
    ///             includes:
    ///               - api-key
    ///               - dnt
    ///           host:
    ///             resolved: true
    ///           queryString:
    ///             ignore: true
    ///           user:
    ///             deviceType: false
    ///             geo: true
    ///             lang: true
    /// ```
    /// <!--End PulumiCodeChooser -->
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheTtlByStatus {
    /// A HTTP code (e.g. `404`) or range of codes (e.g. `400-499`)
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    /// Duration a resource lives in the Cloudflare cache.
    /// - positive number - cache for specified duration in seconds
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsForwardingUrl {
    /// The status code to use for the redirection.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
    /// The URL to which the page rule should forward.
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsMinify {
    /// Whether CSS should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    /// Whether HTML should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    /// Whether Javascript should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    /// Command used to build project.
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    /// Output directory of the build.
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    /// The classifying tag for analytics.
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    /// The auth token for analytics.
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigs {
    /// Configuration for preview deploys.
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<crate::types::PagesProjectDeploymentConfigsPreview>>,
    /// Configuration for production deploys.
    #[serde(rename = "production")]
    pub r#production: Box<Option<crate::types::PagesProjectDeploymentConfigsProduction>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreview {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    /// Compatibility date used for Pages Functions.
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    /// Compatibility flags used for Pages Functions.
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsPreviewPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Services used for Pages Functions.
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
    /// Placement Mode for the Pages Function.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewServiceBinding {
    /// The name of the Worker environment to bind to.
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProduction {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    /// Compatibility date used for Pages Functions.
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    /// Compatibility flags used for Pages Functions.
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsProductionPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Services used for Pages Functions.
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsProductionServiceBinding>>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionPlacement {
    /// Placement Mode for the Pages Function.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionServiceBinding {
    /// The name of the Worker environment to bind to.
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSource {
    /// Configuration for the source of the Cloudflare Pages project.
    #[serde(rename = "config")]
    pub r#config: Box<Option<crate::types::PagesProjectSourceConfig>>,
    /// Project host type.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSourceConfig {
    /// Toggle deployments on this repo. Defaults to `true`.
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Box<Option<bool>>,
    /// Project owner username. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "owner")]
    pub r#owner: Box<Option<String>>,
    /// Enable Pages to comment on Pull Requests. Defaults to `true`.
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Box<Option<bool>>,
    /// Branches will be excluded from automatic deployment.
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Box<Option<Vec<String>>>,
    /// Branches will be included for automatic deployment.
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Box<Option<Vec<String>>>,
    /// Preview Deployment Setting. Available values: `custom`, `all`, `none`. Defaults to `all`.
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Box<Option<String>>,
    /// Project production branch name.
    #[serde(rename = "productionBranch")]
    pub r#production_branch: Box<String>,
    /// Enable production deployments. Defaults to `true`.
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Box<Option<bool>>,
    /// Project repository name. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitAction {
    /// The type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Custom content-type and body to return, this overrides the custom error for the zone. This field is not required. Omission will result in default HTML error page.
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitActionResponse>>,
    /// The time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period.
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitActionResponse {
    /// The body to return, the content here should conform to the `content_type`.
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    /// The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RateLimitCorrelate {
    /// If set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`.
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatch {
    /// Matches HTTP requests (from the client to Cloudflare).
    #[serde(rename = "request")]
    pub r#request: Box<Option<crate::types::RateLimitMatchRequest>>,
    /// Matches HTTP responses before they are returned to the client from Cloudflare. If this is defined, then the entire counting of traffic occurs at this stage.
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitMatchResponse>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchRequest {
    /// HTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`.
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    /// HTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`.
    #[serde(rename = "schemes")]
    pub r#schemes: Box<Option<Vec<String>>>,
    /// The URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone.
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchResponse {
    /// List of HTTP headers maps to match the origin response on.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    /// Only count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting.
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Box<Option<bool>>,
    /// HTTP Status codes, can be one, many or indicate all by not providing this value.
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<i32>>>,
}

#[derive(serde::Serialize)]
pub struct RecordData {
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<i32>>,
    #[serde(rename = "altitude")]
    pub r#altitude: Box<Option<f64>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    #[serde(rename = "digestType")]
    pub r#digest_type: Box<Option<i32>>,
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Box<Option<String>>,
    #[serde(rename = "flags")]
    pub r#flags: Box<Option<String>>,
    #[serde(rename = "keyTag")]
    pub r#key_tag: Box<Option<i32>>,
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Box<Option<i32>>,
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Box<Option<String>>,
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Box<Option<i32>>,
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Box<Option<f64>>,
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Box<Option<i32>>,
    #[serde(rename = "longDirection")]
    pub r#long_direction: Box<Option<String>>,
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Box<Option<i32>>,
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Box<Option<f64>>,
    #[serde(rename = "matchingType")]
    pub r#matching_type: Box<Option<i32>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Box<Option<f64>>,
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Box<Option<f64>>,
    #[serde(rename = "preference")]
    pub r#preference: Box<Option<i32>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    #[serde(rename = "proto")]
    pub r#proto: Box<Option<String>>,
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<i32>>,
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    #[serde(rename = "replacement")]
    pub r#replacement: Box<Option<String>>,
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<i32>>,
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    #[serde(rename = "size")]
    pub r#size: Box<Option<f64>>,
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<i32>>,
    #[serde(rename = "usage")]
    pub r#usage: Box<Option<i32>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::RulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of parameters that configure exposed credential checks.
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Box<Option<crate::types::RulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Unique rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The most recent update to this rule.
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    /// List parameters to configure how the rule generates logs. Only valid for skip action.
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::RulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::RulesetRuleRatelimit>>,
    /// Rule reference.
    #[serde(rename = "ref")]
    pub r#ref: Box<Option<String>>,
    /// Version of the ruleset to deploy.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParameters {
    /// Specifies uncommon ports to allow cacheable assets to be served from.
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    /// Compression algorithms to use in order of preference.
    #[serde(rename = "algorithms")]
    pub r#algorithms: Box<Option<Vec<crate::types::RulesetRuleActionParametersAlgorithm>>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    /// Indicate which file extensions to minify automatically.
    #[serde(rename = "autominifies")]
    pub r#autominifies: Box<Option<Vec<crate::types::RulesetRuleActionParametersAutominify>>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    /// List of browser TTL parameters to apply to the request.
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<crate::types::RulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    /// List of cache key parameters to apply to the request.
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<crate::types::RulesetRuleActionParametersCacheKey>>,
    /// Content of the custom error response.
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// List of cookie values to include as part of custom fields logging.
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    /// Turn off all active Cloudflare Apps.
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Turn off zaraz feature.
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// List of edge TTL parameters to apply to the request.
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<crate::types::RulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    /// Use a list to lookup information for the action.
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<crate::types::RulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<crate::types::RulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule. Note: Headers are order dependent and must be provided sorted alphabetically ascending based on the `name` value.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::RulesetRuleActionParametersHeader>>>,
    /// Host Header that request origin receives.
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    /// Turn on or off the hotlink protection feature.
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    /// Identifier of the action parameter to modify.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    /// List of properties to configure WAF payload logging.
    #[serde(rename = "matchedData")]
    pub r#matched_data: Box<Option<crate::types::RulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    /// List of properties to change request origin.
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<crate::types::RulesetRuleActionParametersOrigin>>,
    /// Enable or disable the use of a more compliant Cache Control parsing mechanism, enabled by default for most zones.
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    /// Pass-through error page for origin.
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    /// List of override configurations to apply to the ruleset.
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<crate::types::RulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`.
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Specifies a maximum timeout for reading content from an origin server.
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    /// Respect strong ETags.
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    /// List of parameters that configure the response given to end users.
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<crate::types::RulesetRuleActionParametersResponse>>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which ruleset ID to target.
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip.
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    /// Control options for the Security Level feature from the Security app.
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// List of serve stale parameters to apply to the request.
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<crate::types::RulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    /// List of properties to manange Server Name Indication.
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<crate::types::RulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Turn on or off the SXG feature.
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<crate::types::RulesetRuleActionParametersUri>>,
    /// Version of the ruleset to deploy.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAlgorithm {
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAutominify {
    /// CSS minification.
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    /// HTML minification.
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    /// JS minification.
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL. This value is required when override_origin is set
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL. Available values: `override_origin`, `respect_origin`, `bypass`
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKey {
    /// Cache by device type.
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    /// Cache deception armor.
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    /// Custom key parameters for the request.
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[serde(rename = "host")]
    pub r#host: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[serde(rename = "queryString")]
    pub r#query_string:
        Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[serde(rename = "user")]
    pub r#user: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyCookie {
    /// List of cookies to check for presence in the custom key.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// List of cookies to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of cookies to check for presence in the custom key.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Exclude the origin header from the custom key.
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    /// List of cookies to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key.
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of cookies to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// Add geo data to the custom key.
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// Add language data to the custom key.
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtl {
    /// Default browser TTL. This value is required when override_origin is set
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL. Available values: `override_origin`, `respect_origin`, `bypass`
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied.
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>,
    >,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    /// From status code.
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    /// To status code.
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<crate::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValueTargetUrl {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`.
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersMatchedData {
    /// Public key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure).
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOrigin {
    /// Host parameters for the custom key.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Origin Port where request is sent.
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverrides {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of tag-based overrides.
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesCategory>>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesRule>>>,
    /// Sensitivity level for a ruleset rule override.
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesCategory {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Tag name to apply the ruleset rule override to.
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Anomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets.
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    /// Sensitivity level for a ruleset rule override.
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersResponse {
    /// Content of the custom error response.
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersServeStale {
    /// Disable stale while updating.
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersSni {
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUri {
    /// List of properties to change request origin.
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::RulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::RulesetRuleActionParametersUriQuery>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriPath {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriQuery {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleExposedCredentialCheck {
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "password" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "username" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleLogging {
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleRatelimit {
    /// List of parameters that define how Cloudflare tracks the request rate for this rule.
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    /// Criteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    /// Once the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field.
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    /// The period of time to consider (in seconds) when evaluating the request rate.
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    /// The number of requests over the period of time that will trigger the Rate Limiting rule.
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    /// Whether to include requests to origin within the Rate Limiting count.
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    /// The maximum aggregate score over the period of time that will trigger Rate Limiting rule.
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    /// Name of HTTP header in the response, set by the origin server, with the score for the current request.
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationDns {
    /// The name of the DNS record associated with the application.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of DNS record associated with the application.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationEdgeIps {
    /// The IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`.
    #[serde(rename = "connectivity")]
    pub r#connectivity: Box<Option<String>>,
    /// The collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    /// The type of edge IP configuration specified. Available values: `dynamic`, `static`.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginDns {
    /// Fully qualified domain name of the origin.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginPortRange {
    /// Upper bound of the origin port range.
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    /// Lower bound of the origin port range.
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct SplitTunnelTunnel {
    /// The address for the tunnel.
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// A description for the tunnel.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The domain name for the tunnel.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirus {
    /// Scan on file download.
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: Box<bool>,
    /// Scan on file upload.
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: Box<bool>,
    /// Block requests for files that cannot be scanned.
    #[serde(rename = "failClosed")]
    pub r#fail_closed: Box<bool>,
    /// Set notifications for antivirus.
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsAccountAntivirusNotificationSettings>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirusNotificationSettings {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Notification content.
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Support URL to show in the notification.
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBlockPage {
    /// Hex code of block page background color.
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// Indicator of enablement.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Block page footer text.
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    /// Block page header text.
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    /// URL of block page logo.
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    /// Admin email for users to contact.
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    /// Subject line for emails created from block page.
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    /// Name of block page configuration.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBodyScanning {
    /// Body scanning inspection mode. Available values: `deep`, `shallow`.
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountExtendedEmailMatching {
    /// Whether e-mails should be matched on all variants of user emails (with + or . modifiers) in Firewall policies.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountFips {
    /// Only allow FIPS-compliant TLS configuration.
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLogging {
    /// Redact personally identifiable information from activity logging (PII fields are: source IP, user email, user ID, device ID, URL, referrer, user agent).
    #[serde(rename = "redactPii")]
    pub r#redact_pii: Box<bool>,
    /// Represents whether all requests are logged or only the blocked requests are slogged in DNS, HTTP and L4 filters.
    #[serde(rename = "settingsByRuleType")]
    pub r#settings_by_rule_type: Box<crate::types::TeamsAccountLoggingSettingsByRuleType>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[serde(rename = "dns")]
    pub r#dns: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[serde(rename = "http")]
    pub r#http: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[serde(rename = "l4")]
    pub r#l_4: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeDns {
    /// Whether to log all activity.
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeHttp {
    /// Whether to log all activity.
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeL4 {
    /// Whether to log all activity.
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountPayloadLog {
    /// Public key used to encrypt matched payloads.
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountProxy {
    /// Whether root ca is enabled account wide for ZT clients.
    #[serde(rename = "rootCa")]
    pub r#root_ca: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for TCP traffic.
    #[serde(rename = "tcp")]
    pub r#tcp: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for UDP traffic.
    #[serde(rename = "udp")]
    pub r#udp: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountSshSessionLog {
    /// Public key used to encrypt ssh session.
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsLocationNetwork {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// CIDR notation representation of the network IP.
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettings {
    /// Add custom headers to allowed requests in the form of key-value pairs.
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Allow parent MSP accounts to enable bypass their children's rules.
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Box<Option<bool>>,
    /// Settings for auditing SSH usage.
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Box<Option<crate::types::TeamsRuleRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Box<Option<crate::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    /// Indicator of block page enablement.
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Box<Option<bool>>,
    /// The displayed reason for a user being blocked.
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Box<Option<String>>,
    /// Allow child MSP accounts to bypass their parent's rule.
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Box<Option<bool>>,
    /// Configure how session check behaves.
    #[serde(rename = "checkSession")]
    pub r#check_session: Box<Option<crate::types::TeamsRuleRuleSettingsCheckSession>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<crate::types::TeamsRuleRuleSettingsEgress>>,
    /// Disable DNSSEC validation (must be Allow rule).
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Box<Option<bool>>,
    /// Turns on IP category based filter on dns if the rule contains dns category checks.
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Box<Option<bool>>,
    /// Settings to forward layer 4 traffic.
    #[serde(rename = "l4override")]
    pub r#l_4_override: Box<Option<crate::types::TeamsRuleRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsRuleRuleSettingsNotificationSettings>>,
    /// The host to override matching DNS queries with.
    #[serde(rename = "overrideHost")]
    pub r#override_host: Box<Option<String>>,
    /// The IPs to override matching DNS queries with.
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Box<Option<Vec<String>>>,
    /// Configure DLP Payload Logging settings for this rule.
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Box<Option<crate::types::TeamsRuleRuleSettingsPayloadLog>>,
    /// Configure untrusted certificate settings for this rule.
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Box<Option<crate::types::TeamsRuleRuleSettingsUntrustedCert>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    /// Log all SSH commands.
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    /// Disable copy-paste.
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    /// Disable download.
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    /// Disable keyboard usage.
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    /// Disable printing.
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    /// Disable upload.
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsCheckSession {
    /// Configure how fresh the session needs to be to be considered valid.
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Enable session enforcement for this rule.
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsEgress {
    /// The IPv4 address to be used for egress.
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<String>,
    /// The IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs.
    #[serde(rename = "ipv4Fallback")]
    pub r#ipv_4_fallback: Box<Option<String>>,
    /// The IPv6 range to be used for egress.
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsL4Override {
    /// Override IP to forward traffic to.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// Override Port to forward traffic to.
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsNotificationSettings {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Notification content.
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Support URL to show in the notification.
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsUntrustedCert {
    /// Action to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfig {
    /// Each incoming request received by cloudflared causes cloudflared to send a request to a local service. This section configures the rules that determine which requests are sent to which local services. [Read more](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/local-management/ingress/).
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Box<Vec<crate::types::TunnelConfigConfigIngressRule>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigOriginRequest>>,
    /// If you're exposing a [private network](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/private-net/), you need to add the `warp-routing` key and set it to `true`.
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Box<Option<crate::types::TunnelConfigConfigWarpRouting>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRule {
    /// Hostname to match the incoming request with. If the hostname matches, the request will be sent to the service.
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>>,
    /// Path of the incoming request. If the path matches, the request will be sent to the local service.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Name of the service to which the request will be sent.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequest {
    /// Access rules for the ingress service.
    #[serde(rename = "access")]
    pub r#access: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequestAccess>>,
    /// Runs as jump host.
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    /// Path to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `""`.
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    /// Timeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`.
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    /// Disables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`.
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    /// Enables HTTP/2 support for the origin connection. Defaults to `false`.
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    /// Sets the HTTP Host header on requests sent to the local service. Defaults to `""`.
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    /// IP rules for the proxy service.
    #[serde(rename = "ipRules")]
    pub r#ip_rules:
        Box<Option<Vec<crate::types::TunnelConfigConfigIngressRuleOriginRequestIpRule>>>,
    /// Maximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`.
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    /// Timeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`.
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    /// Disable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`.
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    /// Disables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`.
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    /// Hostname that cloudflared should expect from your origin server certificate. Defaults to `""`.
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`.
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`.
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `""`, `socks`. Defaults to `""`.
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    /// The timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`.
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    /// Timeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`.
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestAccess {
    /// Audience tags of the access rule.
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    /// Whether the access rule is required.
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// Name of the team to which the access rule applies.
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    /// Ports to use within the IP rule.
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// IP rule prefix.
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequest {
    /// Access rules for the ingress service.
    #[serde(rename = "access")]
    pub r#access: Box<Option<crate::types::TunnelConfigConfigOriginRequestAccess>>,
    /// Runs as jump host.
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    /// Path to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `""`.
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    /// Timeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`.
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    /// Disables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`.
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    /// Enables HTTP/2 support for the origin connection. Defaults to `false`.
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    /// Sets the HTTP Host header on requests sent to the local service. Defaults to `""`.
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    /// IP rules for the proxy service.
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<crate::types::TunnelConfigConfigOriginRequestIpRule>>>,
    /// Maximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`.
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    /// Timeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`.
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    /// Disable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`.
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    /// Disables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`.
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    /// Hostname that cloudflared should expect from your origin server certificate. Defaults to `""`.
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`.
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`.
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `""`, `socks`. Defaults to `""`.
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    /// The timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`.
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    /// Timeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`.
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestAccess {
    /// Audience tags of the access rule.
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    /// Whether the access rule is required.
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// Name of the team to which the access rule applies.
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    /// Ports to use within the IP rule.
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// IP rule prefix.
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigWarpRouting {
    /// Whether WARP routing is enabled.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct UserAgentBlockingRuleConfiguration {
    /// The configuration target for this rule. You must set the target to ua for User Agent Blocking rules.
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The exact user agent string to match. This value will be compared to the received User-Agent HTTP header value.
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomAdditionalRoute {
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomRulesRule {
    /// Action to perform in the ruleset rule. Available values: `bypass_waiting_room`.
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Brief summary of the waiting room rule and its intended use.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Criteria for an HTTP request to trigger the waiting room rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Waiting Room Rules Docs](https://developers.cloudflare.com/waiting-room/additional-options/waiting-room-rules/bypass-rules/).
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Unique rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Whether the rule is enabled or disabled. Available values: `enabled`, `disabled`.
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Version of the waiting room rule.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptAnalyticsEngineBinding {
    /// The name of the Analytics Engine dataset to write to.
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptD1DatabaseBinding {
    /// Database ID of D1 database to use.
    #[serde(rename = "databaseId")]
    pub r#database_id: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptKvNamespaceBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// ID of the KV namespace you want to use.
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlacement {
    /// The placement mode for the Worker. Available values: `smart`.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlainTextBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The plain text you want to store.
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptQueueBinding {
    /// The name of the global variable for the binding in your Worker code.
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// Name of the queue you want to use.
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptR2BucketBinding {
    /// The name of the Bucket to bind to.
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptSecretTextBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The secret text you want to store.
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptServiceBinding {
    /// The name of the Worker environment to bind to.
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptWebassemblyBinding {
    /// The base64 encoded wasm module you want to store.
    #[serde(rename = "module")]
    pub r#module: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneLockdownConfiguration {
    /// The request property to target. Available values: `ip`, `ip_range`.
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The value to target. Depends on target's type. IP addresses should just be standard IPv4/IPv6 notation i.e. `192.0.2.1` or `2001:db8::/32` and IP ranges in CIDR format i.e. `192.0.2.0/24`.
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSetting {
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Box<Option<String>>,
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<String>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Box<Option<String>>,
    #[serde(rename = "brotli")]
    pub r#brotli: Box<Option<String>>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<i32>>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Box<Option<i32>>,
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Box<Option<String>>,
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Box<Option<String>>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Box<Option<String>>,
    #[serde(rename = "fonts")]
    pub r#fonts: Box<Option<String>>,
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Box<Option<String>>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<String>>,
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    #[serde(rename = "http3")]
    pub r#http_3: Box<Option<String>>,
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Box<Option<String>>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<String>>,
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Box<Option<String>>,
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Box<Option<i32>>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    #[serde(rename = "minify")]
    pub r#minify: Box<Option<crate::types::ZoneSettingsOverrideInitialSettingMinify>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect:
        Box<Option<crate::types::ZoneSettingsOverrideInitialSettingMobileRedirect>>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Box<Option<String>>,
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Box<Option<String>>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Box<Option<String>>,
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Box<Option<String>>,
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Box<Option<String>>,
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Box<Option<String>>,
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Box<Option<String>>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    #[serde(rename = "securityHeader")]
    pub r#security_header:
        Box<Option<crate::types::ZoneSettingsOverrideInitialSettingSecurityHeader>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Box<Option<String>>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Box<Option<String>>,
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Box<Option<String>>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Box<Option<String>>,
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Box<Option<String>>,
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
    #[serde(rename = "webp")]
    pub r#webp: Box<Option<String>>,
    #[serde(rename = "websockets")]
    pub r#websockets: Box<Option<String>>,
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingMinify {
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingMobileRedirect {
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: Box<String>,
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[serde(rename = "stripUri")]
    pub r#strip_uri: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingSecurityHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    #[serde(rename = "nosniff")]
    pub r#nosniff: Box<Option<bool>>,
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettings {
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Box<Option<String>>,
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<String>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Box<Option<String>>,
    #[serde(rename = "brotli")]
    pub r#brotli: Box<Option<String>>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<i32>>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Box<Option<i32>>,
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Box<Option<String>>,
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Box<Option<String>>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Box<Option<String>>,
    #[serde(rename = "fonts")]
    pub r#fonts: Box<Option<String>>,
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Box<Option<String>>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<String>>,
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    #[serde(rename = "http3")]
    pub r#http_3: Box<Option<String>>,
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Box<Option<String>>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<String>>,
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Box<Option<String>>,
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Box<Option<i32>>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    #[serde(rename = "minify")]
    pub r#minify: Box<Option<crate::types::ZoneSettingsOverrideSettingsMinify>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Box<Option<crate::types::ZoneSettingsOverrideSettingsMobileRedirect>>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Box<Option<String>>,
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Box<Option<String>>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Box<Option<String>>,
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Box<Option<String>>,
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Box<Option<String>>,
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Box<Option<String>>,
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Box<Option<String>>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    #[serde(rename = "securityHeader")]
    pub r#security_header: Box<Option<crate::types::ZoneSettingsOverrideSettingsSecurityHeader>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Box<Option<String>>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Box<Option<String>>,
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Box<Option<String>>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Box<Option<String>>,
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Box<Option<String>>,
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
    #[serde(rename = "webp")]
    pub r#webp: Box<Option<String>>,
    #[serde(rename = "websockets")]
    pub r#websockets: Box<Option<String>>,
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsMinify {
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsMobileRedirect {
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: Box<String>,
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[serde(rename = "stripUri")]
    pub r#strip_uri: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsSecurityHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    #[serde(rename = "nosniff")]
    pub r#nosniff: Box<Option<bool>>,
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetAccountRolesRole {
    /// Description of role's permissions.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Role identifier tag.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Role Name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetAccountsAccount {
    /// Whether 2FA is enforced on the account.
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Box<Option<bool>>,
    /// Account ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Account name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Account subscription type.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetDevicePostureRulesRule {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    /// ID of the Device Posture Rule.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the device posture rule.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetDevicesDevice {
    /// When the device was created.
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    /// Whether the device has been deleted.
    #[serde(rename = "deleted")]
    pub r#deleted: Box<Option<bool>>,
    /// The type of the device.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<String>>,
    /// Device ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// IPv4 or IPv6 address.
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// The device's public key.
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// When the device was last seen.
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The device's MAC address.
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    /// The device manufacturer's name.
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<Option<String>>,
    /// The device model name.
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    /// The device name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The Linux distribution name.
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The Linux distribution revision.
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// The operating system version.
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
    /// When the device was revoked.
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    /// The device's serial number.
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// When the device was updated.
    #[serde(rename = "updated")]
    pub r#updated: Box<Option<String>>,
    /// User's email.
    #[serde(rename = "userEmail")]
    pub r#user_email: Box<Option<String>>,
    /// User's ID.
    #[serde(rename = "userId")]
    pub r#user_id: Box<Option<String>>,
    /// User's Name.
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
    /// The WARP client version.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetDlpDatasetsDataset {
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "secret")]
    pub r#secret: Box<bool>,
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetListsList {
    /// List description.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// List identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// List kind.
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// The list name to target for the resource.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Number of items in list.
    #[serde(rename = "numitems")]
    pub r#numitems: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsFilter {
    /// A regular expression matching the name of the Load Balancer pool to lookup.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPool {
    /// List of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://support.cloudflare.com/hc/en-us/articles/115000540888-Load-Balancing-Geographic-Regions).
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Box<Vec<String>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    #[serde(rename = "createdOn")]
    pub r#created_on: Box<String>,
    /// Brief description of the Load Balancer Pool intention.
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Whether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// ID for this load balancer pool.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Latitude this pool is physically located at; used for proximity steering.
    #[serde(rename = "latitude")]
    pub r#latitude: Box<f64>,
    /// Setting for controlling load shedding for this pool.
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Box<Vec<crate::types::GetLoadBalancerPoolsPoolLoadShedding>>,
    /// Longitude this pool is physically located at; used for proximity steering.
    #[serde(rename = "longitude")]
    pub r#longitude: Box<f64>,
    /// Minimum number of origins that must be healthy for this pool to serve traffic.
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: Box<i32>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: Box<String>,
    /// ID of the Monitor to use for health checking origins within this pool.
    #[serde(rename = "monitor")]
    pub r#monitor: Box<String>,
    /// Short name (tag) for the pool.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Email address to send health status notifications to. Multiple emails are set as a comma delimited list.
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: Box<String>,
    /// The list of origins within this pool.
    #[serde(rename = "origins")]
    pub r#origins: Box<Vec<crate::types::GetLoadBalancerPoolsPoolOrigin>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100.
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    /// Percent of session traffic to shed 0 - 100.
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolOrigin {
    /// The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname.
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Whether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// HTTP request headers.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::GetLoadBalancerPoolsPoolOriginHeader>>>,
    /// A regular expression matching the name of the Load Balancer pool to lookup.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. When `origin_steering.policy="least_outstanding_requests"`, weight is used to scale the origin's outstanding requests. When `origin_steering.policy="least_connections"`, weight is used to scale the origin's open connections.
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    /// HTTP Header name.
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Values for the HTTP headers.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsFilter {
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// Name of the ruleset.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[serde(rename = "phase")]
    pub r#phase: Box<Option<String>>,
    /// Version of the ruleset to filter on.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRuleset {
    /// Brief summary of the ruleset and its intended use.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// ID of the ruleset.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Type of Ruleset. Available values: `custom`, `managed`, `root`, `zone`
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    /// Name of the ruleset.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Point in the request/response lifecycle where the ruleset executes. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`
    #[serde(rename = "phase")]
    pub r#phase: Box<String>,
    /// List of rules to apply to the ruleset.
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::GetRulesetsRulesetRule>>>,
    /// Version of the ruleset.
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::GetRulesetsRulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of parameters that configure exposed credential checks.
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check:
        Box<Option<crate::types::GetRulesetsRulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The most recent update to this rule.
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    /// List parameters to configure how the rule generates logs.
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::GetRulesetsRulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::GetRulesetsRulesetRuleRatelimit>>,
    /// Rule reference.
    #[serde(rename = "ref")]
    pub r#ref: Box<String>,
    /// Version of the ruleset to filter on.
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParameters {
    /// Allows for the ability to support caching on non-standard ports.
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    /// Indicate which file extensions to minify automatically.
    #[serde(rename = "autominifies")]
    pub r#autominifies:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersAutominify>>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    /// List of browser TTL parameters to apply to the request.
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    /// List of cache key parameters to apply to the request.
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKey>>,
    /// Content of the custom error response
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// List of cookie values to include as part of custom fields logging.
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    /// Turn off all active Cloudflare Apps.
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Turn off zaraz feature.
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// List of edge TTL parameters to apply to the request.
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    /// Use a list to lookup information for the action.
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersHeader>>>,
    /// Host Header that request origin receives.
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    /// Turn on or off the hotlink protection feature.
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    /// List of properties to configure WAF payload logging.
    #[serde(rename = "matchedData")]
    pub r#matched_data:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    /// List of properties to change request origin.
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOrigin>>,
    /// Sets a more compliant mode for parsing Cache Control headers
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    /// Pass-through error page for origin.
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    /// List of override configurations to apply to the ruleset.
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Sets the timeout value for reading content from an origin server.
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    /// Respect strong ETags.
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    /// List of parameters that configure the response given to end users
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersResponse>>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    /// Map of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { "efb7b8c949ac4650a09736fc376e9aee" = "5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760" }`
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which ruleset ID to target.
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    /// Control options for the Security Level feature from the Security app.
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// List of serve stale parameters to apply to the request.
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    /// List of properties to manange Server Name Indication.
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// HTTP status code of the custom error response
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Turn on or off the SXG feature.
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUri>>,
    /// Version of the ruleset to filter on.
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersAutominify {
    /// SSL minification.
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    /// HTML minification.
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    /// JS minification.
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL.
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKey {
    /// Cache by device type. Conflicts with "custom_key.user.device_type".
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    /// Cache deception armor.
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    /// Custom key parameters for the request.
    #[serde(rename = "customKey")]
    pub r#custom_key:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[serde(rename = "cookie")]
    pub r#cookie:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[serde(rename = "header")]
    pub r#header:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[serde(rename = "host")]
    pub r#host:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[serde(rename = "queryString")]
    pub r#query_string: Box<
        Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>,
    >,
    /// User parameters for the custom key.
    #[serde(rename = "user")]
    pub r#user:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie {
    /// List of cookies to check for presence in the custom key.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// List of cookies to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of headers to check for presence in the custom key.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Exclude the origin header from the custom key.
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    /// List of headers to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key. Conflicts with "include".
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key. Conflicts with "exclude".
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key. Conflicts with "cache_key.cache_by_device_type".
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// Add geo data to the custom key.
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// Add language data to the custom key.
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtl {
    /// Default edge TTL
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the edge TTL.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied. Conflicts with "status_code_range".
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied. Conflicts with "status_code".
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    /// From status code.
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    /// To status code.
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Name of the ruleset.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for redirect.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValueTargetUrl>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromValueTargetUrl {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions. Conflicts with `"value"`.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Static value to provide as the HTTP request header value. Conflicts with `"expression"`.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions. Conflicts with `"value"`.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Name of the ruleset.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// Static value to provide as the HTTP request header value. Conflicts with `"expression"`.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersMatchedData {
    /// Public key to use within WAF Ruleset payload logging to view the HTTP request parameters. You can generate a public key [using the `matched-data-cli` command-line tool](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/command-line/generate-key-pair) or [in the Cloudflare dashboard](https://developers.cloudflare.com/waf/managed-rulesets/payload-logging/configure)
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOrigin {
    /// Origin Hostname where request is sent.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Origin Port where request is sent.
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverrides {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of tag-based overrides.
    #[serde(rename = "categories")]
    pub r#categories:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesCategory>>>,
    /// Defines if the current ruleset-level override enables or disables the ruleset.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[serde(rename = "rules")]
    pub r#rules:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesRule>>>,
    /// Sensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    /// Defines if the current ruleset-level override enables or disables the ruleset. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesCategory {
    /// Action to perform in the tag-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Tag name to apply the ruleset rule override to.
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesRule {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Defines if the current rule-level override enables or disables the rule.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Anomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets.
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    /// Sensitivity level for a ruleset rule override.
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    /// Defines if the current rule-level override enables or disables the rule. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    /// Body content to include in the response.
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// HTTP content type to send in the response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// HTTP status code to send in the response.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersServeStale {
    /// Disable stale while updating.
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersSni {
    /// Value to define for SNI.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriQuery>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUriPath {
    /// Expression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Static string value of the updated URI path or query string component.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUriQuery {
    /// Expression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Static string value of the updated URI path or query string component.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleExposedCredentialCheck {
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "password" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "username" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleLogging {
    /// Override the default logging behavior when a rule is matched.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Override the default logging behavior when a rule is matched. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleRatelimit {
    /// List of parameters that define how Cloudflare tracks the request rate for this rule.
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    /// Criteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    /// Once the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field.
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    /// The period of time to consider (in seconds) when evaluating the request rate.
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    /// The number of requests over the period of time that will trigger the Rate Limiting rule.
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    /// Whether to include requests to origin within the Rate Limiting count.
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    /// The maximum aggregate score over the period of time that will trigger Rate Limiting rule.
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    /// Name of HTTP header in the response, set by the origin server, with the score for the current request.
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetZonesFilter {
    /// The account identifier to target for the resource.
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// The type of search to perform for the `name` value when querying the zone API. Available values: `contains`, `exact`. Defaults to `exact`.
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Box<Option<String>>,
    /// A RE2 compatible regular expression to filter the	results. This is performed client side whereas the `name` and `lookup_type`	are performed on the Cloudflare server side.
    #[serde(rename = "match")]
    pub r#match: Box<Option<String>>,
    /// A string value to search for.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Paused status of the zone to lookup. Defaults to `false`.
    #[serde(rename = "paused")]
    pub r#paused: Box<Option<bool>>,
    /// Status of the zone to lookup.
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetZonesZone {
    /// The zone ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Zone name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
