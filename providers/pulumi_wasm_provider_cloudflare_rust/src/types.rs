#[derive(serde::Serialize)]
pub struct AccessApplicationCorsHeader {
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Option<bool>,
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Option<bool>,
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Option<bool>,
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationFooterLink {
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationLandingPageDesign {
    #[serde(rename = "buttonColor")]
    pub r#button_color: Option<String>,
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Option<String>,
    #[serde(rename = "imageUrl")]
    pub r#image_url: Option<String>,
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasApp {
    #[serde(rename = "appLauncherUrl")]
    pub r#app_launcher_url: Option<String>,
    #[serde(rename = "authType")]
    pub r#auth_type: Option<String>,
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[serde(rename = "consumerServiceUrl")]
    pub r#consumer_service_url: Option<String>,
    #[serde(rename = "customAttributes")]
    pub r#custom_attributes: Option<Vec<crate::types::AccessApplicationSaasAppCustomAttribute>>,
    #[serde(rename = "defaultRelayState")]
    pub r#default_relay_state: Option<String>,
    #[serde(rename = "grantTypes")]
    pub r#grant_types: Option<Vec<String>>,
    #[serde(rename = "groupFilterRegex")]
    pub r#group_filter_regex: Option<String>,
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: Option<String>,
    #[serde(rename = "nameIdFormat")]
    pub r#name_id_format: Option<String>,
    #[serde(rename = "nameIdTransformJsonata")]
    pub r#name_id_transform_jsonata: Option<String>,
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    #[serde(rename = "redirectUris")]
    pub r#redirect_uris: Option<Vec<String>>,
    #[serde(rename = "samlAttributeTransformJsonata")]
    pub r#saml_attribute_transform_jsonata: Option<String>,
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Option<String>,
    #[serde(rename = "ssoEndpoint")]
    pub r#sso_endpoint: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttribute {
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "nameFormat")]
    pub r#name_format: Option<String>,
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    #[serde(rename = "source")]
    pub r#source: crate::types::AccessApplicationSaasAppCustomAttributeSource,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttributeSource {
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessGroupExcludeAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessGroupExcludeAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessGroupExcludeExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessGroupExcludeGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessGroupExcludeGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessGroupExcludeOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessGroupExcludeSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupInclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessGroupIncludeAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessGroupIncludeAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessGroupIncludeExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessGroupIncludeGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessGroupIncludeGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessGroupIncludeOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessGroupIncludeSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequire {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessGroupRequireAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessGroupRequireAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessGroupRequireExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessGroupRequireGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessGroupRequireGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessGroupRequireOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessGroupRequireSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessIdentityProviderConfig {
    #[serde(rename = "apiToken")]
    pub r#api_token: Option<String>,
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Option<String>,
    #[serde(rename = "attributes")]
    pub r#attributes: Option<Vec<String>>,
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Option<String>,
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Option<String>,
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Option<String>,
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Option<String>,
    #[serde(rename = "claims")]
    pub r#claims: Option<Vec<String>>,
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Option<bool>,
    #[serde(rename = "directoryId")]
    pub r#directory_id: Option<String>,
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Option<String>,
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Option<String>,
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Option<String>,
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Option<String>,
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Option<String>,
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Option<String>,
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Option<String>,
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Option<bool>,
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Option<String>,
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    #[serde(rename = "signRequest")]
    pub r#sign_request: Option<bool>,
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Option<String>,
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Option<bool>,
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessIdentityProviderScimConfig {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "groupMemberDeprovision")]
    pub r#group_member_deprovision: Option<bool>,
    #[serde(rename = "seatDeprovision")]
    pub r#seat_deprovision: Option<bool>,
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
    #[serde(rename = "userDeprovision")]
    pub r#user_deprovision: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct AccessMutualTlsHostnameSettingsSetting {
    #[serde(rename = "chinaNetwork")]
    pub r#china_network: Option<bool>,
    #[serde(rename = "clientCertificateForwarding")]
    pub r#client_certificate_forwarding: Option<bool>,
    #[serde(rename = "hostname")]
    pub r#hostname: String,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationCustomPage {
    #[serde(rename = "forbidden")]
    pub r#forbidden: Option<String>,
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationLoginDesign {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Option<String>,
    #[serde(rename = "headerText")]
    pub r#header_text: Option<String>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Option<String>,
    #[serde(rename = "textColor")]
    pub r#text_color: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyApprovalGroup {
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: i32,
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Option<Vec<String>>,
    #[serde(rename = "emailListUuid")]
    pub r#email_list_uuid: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessPolicyExcludeAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessPolicyExcludeAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessPolicyExcludeExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessPolicyExcludeGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessPolicyExcludeGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessPolicyExcludeOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessPolicyExcludeSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyInclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessPolicyIncludeAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessPolicyIncludeAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessPolicyIncludeExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessPolicyIncludeGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessPolicyIncludeGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessPolicyIncludeOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessPolicyIncludeSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequire {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<crate::types::AccessPolicyRequireAuthContext>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<crate::types::AccessPolicyRequireAzure>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<crate::types::AccessPolicyRequireExternalEvaluation>,
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<crate::types::AccessPolicyRequireGithub>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<crate::types::AccessPolicyRequireGsuite>>,
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<crate::types::AccessPolicyRequireOkta>>,
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<crate::types::AccessPolicyRequireSaml>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: String,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Option<String>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "teams")]
    pub r#teams: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireOkta {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
    #[serde(rename = "names")]
    pub r#names: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AccessRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: String,
    #[serde(rename = "value")]
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct AddressMapIp {
    #[serde(rename = "ip")]
    pub r#ip: String,
}

#[derive(serde::Serialize)]
pub struct AddressMapMembership {
    #[serde(rename = "canDelete")]
    pub r#can_delete: Option<bool>,
    #[serde(rename = "identifier")]
    pub r#identifier: String,
    #[serde(rename = "kind")]
    pub r#kind: String,
}

#[derive(serde::Serialize)]
pub struct ApiShieldAuthIdCharacteristic {
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenCondition {
    #[serde(rename = "requestIp")]
    pub r#request_ip: Option<crate::types::ApiTokenConditionRequestIp>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenConditionRequestIp {
    #[serde(rename = "ins")]
    pub r#ins: Option<Vec<String>>,
    #[serde(rename = "notIns")]
    pub r#not_ins: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenPolicy {
    #[serde(rename = "effect")]
    pub r#effect: Option<String>,
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Vec<String>,
    #[serde(rename = "resources")]
    pub r#resources: std::collections::HashMap<String, String>,
}

#[derive(serde::Serialize)]
pub struct CertificatePackValidationError {
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CertificatePackValidationRecord {
    #[serde(rename = "cnameName")]
    pub r#cname_name: Option<String>,
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Option<String>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "httpBody")]
    pub r#http_body: Option<String>,
    #[serde(rename = "httpUrl")]
    pub r#http_url: Option<String>,
    #[serde(rename = "txtName")]
    pub r#txt_name: Option<String>,
    #[serde(rename = "txtValue")]
    pub r#txt_value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSsl {
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Option<String>,
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Option<String>,
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Option<String>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<String>,
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    #[serde(rename = "settings")]
    pub r#settings: Option<Vec<crate::types::CustomHostnameSslSetting>>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Option<Vec<crate::types::CustomHostnameSslValidationError>>,
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Option<Vec<crate::types::CustomHostnameSslValidationRecord>>,
    #[serde(rename = "wildcard")]
    pub r#wildcard: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslSetting {
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslValidationError {
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslValidationRecord {
    #[serde(rename = "cnameName")]
    pub r#cname_name: Option<String>,
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Option<String>,
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[serde(rename = "httpBody")]
    pub r#http_body: Option<String>,
    #[serde(rename = "httpUrl")]
    pub r#http_url: Option<String>,
    #[serde(rename = "txtName")]
    pub r#txt_name: Option<String>,
    #[serde(rename = "txtValue")]
    pub r#txt_value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CustomSslCustomSslOptions {
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Option<String>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    #[serde(rename = "geoRestrictions")]
    pub r#geo_restrictions: Option<String>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CustomSslCustomSslPriority {
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct DeviceDexTestData {
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "kind")]
    pub r#kind: String,
    #[serde(rename = "method")]
    pub r#method: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DeviceManagedNetworksConfig {
    #[serde(rename = "sha256")]
    pub r#sha_256: String,
    #[serde(rename = "tlsSockaddr")]
    pub r#tls_sockaddr: String,
}

#[derive(serde::Serialize)]
pub struct DevicePostureIntegrationConfig {
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Option<String>,
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Option<String>,
    #[serde(rename = "apiUrl")]
    pub r#api_url: Option<String>,
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<String>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[serde(rename = "customerId")]
    pub r#customer_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleInput {
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Option<i32>,
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Option<String>,
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Option<Vec<String>>,
    #[serde(rename = "cn")]
    pub r#cn: Option<String>,
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Option<String>,
    #[serde(rename = "connectionId")]
    pub r#connection_id: Option<String>,
    #[serde(rename = "countOperator")]
    pub r#count_operator: Option<String>,
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "exists")]
    pub r#exists: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "infected")]
    pub r#infected: Option<bool>,
    #[serde(rename = "isActive")]
    pub r#is_active: Option<bool>,
    #[serde(rename = "issueCount")]
    pub r#issue_count: Option<String>,
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Option<String>,
    #[serde(rename = "networkStatus")]
    pub r#network_status: Option<String>,
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    #[serde(rename = "os")]
    pub r#os: Option<String>,
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Option<String>,
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Option<String>,
    #[serde(rename = "overall")]
    pub r#overall: Option<String>,
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    #[serde(rename = "requireAll")]
    pub r#require_all: Option<bool>,
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Option<String>,
    #[serde(rename = "running")]
    pub r#running: Option<bool>,
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Option<String>,
    #[serde(rename = "sha256")]
    pub r#sha_256: Option<String>,
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Option<String>,
    #[serde(rename = "totalScore")]
    pub r#total_score: Option<i32>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleMatch {
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwareness {
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[serde(rename = "skip")]
    pub r#skip: crate::types::DlpProfileContextAwarenessSkip,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwarenessSkip {
    #[serde(rename = "files")]
    pub r#files: bool,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntry {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "pattern")]
    pub r#pattern: Option<crate::types::DlpProfileEntryPattern>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntryPattern {
    #[serde(rename = "regex")]
    pub r#regex: String,
    #[serde(rename = "validation")]
    pub r#validation: Option<String>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllMatcher {
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "values")]
    pub r#values: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleMatcher {
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct FallbackDomainDomain {
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}

#[derive(serde::Serialize)]
pub struct HealthcheckHeader {
    #[serde(rename = "header")]
    pub r#header: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigCaching {
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigOrigin {
    #[serde(rename = "database")]
    pub r#database: String,
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "password")]
    pub r#password: String,
    #[serde(rename = "port")]
    pub r#port: i32,
    #[serde(rename = "scheme")]
    pub r#scheme: String,
    #[serde(rename = "user")]
    pub r#user: String,
}

#[derive(serde::Serialize)]
pub struct ListItem {
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    #[serde(rename = "value")]
    pub r#value: crate::types::ListItemValue,
}

#[derive(serde::Serialize)]
pub struct ListItemHostname {
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: String,
}

#[derive(serde::Serialize)]
pub struct ListItemRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Option<bool>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<bool>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: String,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Option<bool>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}

#[derive(serde::Serialize)]
pub struct ListItemValue {
    #[serde(rename = "asn")]
    pub r#asn: Option<i32>,
    #[serde(rename = "hostnames")]
    pub r#hostnames: Option<Vec<crate::types::ListItemValueHostname>>,
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    #[serde(rename = "redirects")]
    pub r#redirects: Option<Vec<crate::types::ListItemValueRedirect>>,
}

#[derive(serde::Serialize)]
pub struct ListItemValueHostname {
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: String,
}

#[derive(serde::Serialize)]
pub struct ListItemValueRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<String>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Option<String>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: String,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Option<String>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerCountryPool {
    #[serde(rename = "country")]
    pub r#country: String,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerLocationStrategy {
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerMonitorHeader {
    #[serde(rename = "header")]
    pub r#header: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolLoadShedding {
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Option<f64>,
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Option<String>,
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Option<f64>,
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOrigin {
    #[serde(rename = "address")]
    pub r#address: String,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<crate::types::LoadBalancerPoolOriginHeader>>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "weight")]
    pub r#weight: Option<f64>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginHeader {
    #[serde(rename = "header")]
    pub r#header: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginSteering {
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPopPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
    #[serde(rename = "pop")]
    pub r#pop: String,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRandomSteering {
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Option<f64>,
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Option<std::collections::HashMap<String, f64>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRegionPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
    #[serde(rename = "region")]
    pub r#region: String,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRule {
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<crate::types::LoadBalancerRuleFixedResponse>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "overrides")]
    pub r#overrides: Option<Vec<crate::types::LoadBalancerRuleOverride>>,
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    #[serde(rename = "terminates")]
    pub r#terminates: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleFixedResponse {
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    #[serde(rename = "messageBody")]
    pub r#message_body: Option<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverride {
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings: Option<Vec<crate::types::LoadBalancerRuleOverrideAdaptiveRouting>>,
    #[serde(rename = "countryPools")]
    pub r#country_pools: Option<Vec<crate::types::LoadBalancerRuleOverrideCountryPool>>,
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Option<Vec<String>>,
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Option<String>,
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies: Option<Vec<crate::types::LoadBalancerRuleOverrideLocationStrategy>>,
    #[serde(rename = "popPools")]
    pub r#pop_pools: Option<Vec<crate::types::LoadBalancerRuleOverridePopPool>>,
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Option<Vec<crate::types::LoadBalancerRuleOverrideRandomSteering>>,
    #[serde(rename = "regionPools")]
    pub r#region_pools: Option<Vec<crate::types::LoadBalancerRuleOverrideRegionPool>>,
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<String>,
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes:
        Option<Vec<crate::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>,
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Option<i32>,
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Option<String>,
    #[serde(rename = "ttl")]
    pub r#ttl: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideCountryPool {
    #[serde(rename = "country")]
    pub r#country: String,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideLocationStrategy {
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverridePopPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
    #[serde(rename = "pop")]
    pub r#pop: String,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRandomSteering {
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Option<f64>,
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Option<std::collections::HashMap<String, f64>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRegionPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Vec<String>,
    #[serde(rename = "region")]
    pub r#region: String,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideSessionAffinityAttribute {
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<String>>,
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Option<bool>,
    #[serde(rename = "samesite")]
    pub r#samesite: Option<String>,
    #[serde(rename = "secure")]
    pub r#secure: Option<String>,
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerSessionAffinityAttribute {
    #[serde(rename = "drainDuration")]
    pub r#drain_duration: Option<i32>,
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<String>>,
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Option<bool>,
    #[serde(rename = "samesite")]
    pub r#samesite: Option<String>,
    #[serde(rename = "secure")]
    pub r#secure: Option<String>,
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LogpushJobOutputOptions {
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Option<String>,
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Option<String>,
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Option<bool>,
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Option<String>,
    #[serde(rename = "fieldNames")]
    pub r#field_names: Option<Vec<String>>,
    #[serde(rename = "outputType")]
    pub r#output_type: Option<String>,
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Option<String>,
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Option<String>,
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Option<String>,
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Option<String>,
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedRequestHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[serde(rename = "id")]
    pub r#id: String,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedResponseHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[serde(rename = "id")]
    pub r#id: String,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyEmailIntegration {
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyFilters {
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<String>>,
    #[serde(rename = "affectedComponents")]
    pub r#affected_components: Option<Vec<String>>,
    #[serde(rename = "airportCodes")]
    pub r#airport_codes: Option<Vec<String>>,
    #[serde(rename = "alertTriggerPreferences")]
    pub r#alert_trigger_preferences: Option<Vec<String>>,
    #[serde(rename = "enableds")]
    pub r#enableds: Option<Vec<String>>,
    #[serde(rename = "environments")]
    pub r#environments: Option<Vec<String>>,
    #[serde(rename = "eventSources")]
    pub r#event_sources: Option<Vec<String>>,
    #[serde(rename = "eventTypes")]
    pub r#event_types: Option<Vec<String>>,
    #[serde(rename = "events")]
    pub r#events: Option<Vec<String>>,
    #[serde(rename = "groupBies")]
    pub r#group_bies: Option<Vec<String>>,
    #[serde(rename = "healthCheckIds")]
    pub r#health_check_ids: Option<Vec<String>>,
    #[serde(rename = "incidentImpacts")]
    pub r#incident_impacts: Option<Vec<String>>,
    #[serde(rename = "inputIds")]
    pub r#input_ids: Option<Vec<String>>,
    #[serde(rename = "limits")]
    pub r#limits: Option<Vec<String>>,
    #[serde(rename = "megabitsPerSeconds")]
    pub r#megabits_per_seconds: Option<Vec<String>>,
    #[serde(rename = "newHealths")]
    pub r#new_healths: Option<Vec<String>>,
    #[serde(rename = "newStatuses")]
    pub r#new_statuses: Option<Vec<String>>,
    #[serde(rename = "packetsPerSeconds")]
    pub r#packets_per_seconds: Option<Vec<String>>,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Option<Vec<String>>,
    #[serde(rename = "products")]
    pub r#products: Option<Vec<String>>,
    #[serde(rename = "projectIds")]
    pub r#project_ids: Option<Vec<String>>,
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<String>>,
    #[serde(rename = "requestsPerSeconds")]
    pub r#requests_per_seconds: Option<Vec<String>>,
    #[serde(rename = "selectors")]
    pub r#selectors: Option<Vec<String>>,
    #[serde(rename = "services")]
    pub r#services: Option<Vec<String>>,
    #[serde(rename = "slos")]
    pub r#slos: Option<Vec<String>>,
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<String>>,
    #[serde(rename = "targetHostnames")]
    pub r#target_hostnames: Option<Vec<String>>,
    #[serde(rename = "targetZoneNames")]
    pub r#target_zone_names: Option<Vec<String>>,
    #[serde(rename = "tunnelIds")]
    pub r#tunnel_ids: Option<Vec<String>>,
    #[serde(rename = "wheres")]
    pub r#wheres: Option<Vec<String>>,
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyPagerdutyIntegration {
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyWebhooksIntegration {
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActions {
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Option<bool>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<String>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Option<String>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Option<String>,
    #[serde(rename = "bypassCacheOnCookie")]
    pub r#bypass_cache_on_cookie: Option<String>,
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<String>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<String>,
    #[serde(rename = "cacheKeyFields")]
    pub r#cache_key_fields: Option<crate::types::PageRuleActionsCacheKeyFields>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Option<String>,
    #[serde(rename = "cacheOnCookie")]
    pub r#cache_on_cookie: Option<String>,
    #[serde(rename = "cacheTtlByStatuses")]
    pub r#cache_ttl_by_statuses: Option<Vec<crate::types::PageRuleActionsCacheTtlByStatus>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Option<bool>,
    #[serde(rename = "disablePerformance")]
    pub r#disable_performance: Option<bool>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Option<bool>,
    #[serde(rename = "disableSecurity")]
    pub r#disable_security: Option<bool>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Option<bool>,
    #[serde(rename = "edgeCacheTtl")]
    pub r#edge_cache_ttl: Option<i32>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<String>,
    #[serde(rename = "explicitCacheControl")]
    pub r#explicit_cache_control: Option<String>,
    #[serde(rename = "forwardingUrl")]
    pub r#forwarding_url: Option<crate::types::PageRuleActionsForwardingUrl>,
    #[serde(rename = "hostHeaderOverride")]
    pub r#host_header_override: Option<String>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Option<String>,
    #[serde(rename = "minifies")]
    pub r#minifies: Option<Vec<crate::types::PageRuleActionsMinify>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<String>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Option<String>,
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[serde(rename = "resolveOverride")]
    pub r#resolve_override: Option<String>,
    #[serde(rename = "respectStrongEtag")]
    pub r#respect_strong_etag: Option<String>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Option<String>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<String>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Option<String>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Option<String>,
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Option<String>,
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFields {
    #[serde(rename = "cookie")]
    pub r#cookie: Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>,
    #[serde(rename = "header")]
    pub r#header: Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>,
    #[serde(rename = "host")]
    pub r#host: crate::types::PageRuleActionsCacheKeyFieldsHost,
    #[serde(rename = "queryString")]
    pub r#query_string: crate::types::PageRuleActionsCacheKeyFieldsQueryString,
    #[serde(rename = "user")]
    pub r#user: crate::types::PageRuleActionsCacheKeyFieldsUser,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    #[serde(rename = "ignore")]
    pub r#ignore: Option<bool>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheTtlByStatus {
    #[serde(rename = "codes")]
    pub r#codes: String,
    #[serde(rename = "ttl")]
    pub r#ttl: i32,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsForwardingUrl {
    #[serde(rename = "statusCode")]
    pub r#status_code: i32,
    #[serde(rename = "url")]
    pub r#url: String,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsMinify {
    #[serde(rename = "css")]
    pub r#css: String,
    #[serde(rename = "html")]
    pub r#html: String,
    #[serde(rename = "js")]
    pub r#js: String,
}

#[derive(serde::Serialize)]
pub struct PagesProjectBuildConfig {
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Option<bool>,
    #[serde(rename = "buildCommand")]
    pub r#build_command: Option<String>,
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Option<String>,
    #[serde(rename = "rootDir")]
    pub r#root_dir: Option<String>,
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Option<String>,
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigs {
    #[serde(rename = "preview")]
    pub r#preview: Option<crate::types::PagesProjectDeploymentConfigsPreview>,
    #[serde(rename = "production")]
    pub r#production: Option<crate::types::PagesProjectDeploymentConfigsProduction>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreview {
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Option<bool>,
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Option<String>,
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Option<Vec<String>>,
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "placement")]
    pub r#placement: Option<crate::types::PagesProjectDeploymentConfigsPreviewPlacement>,
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Option<Vec<crate::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>,
    #[serde(rename = "usageModel")]
    pub r#usage_model: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Option<String>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "service")]
    pub r#service: String,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProduction {
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Option<bool>,
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Option<String>,
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Option<Vec<String>>,
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "placement")]
    pub r#placement: Option<crate::types::PagesProjectDeploymentConfigsProductionPlacement>,
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Option<Vec<crate::types::PagesProjectDeploymentConfigsProductionServiceBinding>>,
    #[serde(rename = "usageModel")]
    pub r#usage_model: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Option<String>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "service")]
    pub r#service: String,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSource {
    #[serde(rename = "config")]
    pub r#config: Option<crate::types::PagesProjectSourceConfig>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSourceConfig {
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Option<bool>,
    #[serde(rename = "owner")]
    pub r#owner: Option<String>,
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Option<bool>,
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Option<Vec<String>>,
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Option<Vec<String>>,
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Option<String>,
    #[serde(rename = "productionBranch")]
    pub r#production_branch: String,
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Option<bool>,
    #[serde(rename = "repoName")]
    pub r#repo_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RateLimitAction {
    #[serde(rename = "mode")]
    pub r#mode: String,
    #[serde(rename = "response")]
    pub r#response: Option<crate::types::RateLimitActionResponse>,
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RateLimitActionResponse {
    #[serde(rename = "body")]
    pub r#body: String,
    #[serde(rename = "contentType")]
    pub r#content_type: String,
}

#[derive(serde::Serialize)]
pub struct RateLimitCorrelate {
    #[serde(rename = "by")]
    pub r#by: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatch {
    #[serde(rename = "request")]
    pub r#request: Option<crate::types::RateLimitMatchRequest>,
    #[serde(rename = "response")]
    pub r#response: Option<crate::types::RateLimitMatchResponse>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchRequest {
    #[serde(rename = "methods")]
    pub r#methods: Option<Vec<String>>,
    #[serde(rename = "schemes")]
    pub r#schemes: Option<Vec<String>>,
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchResponse {
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Option<bool>,
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<i32>>,
}

#[derive(serde::Serialize)]
pub struct RecordData {
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<i32>,
    #[serde(rename = "altitude")]
    pub r#altitude: Option<f64>,
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    #[serde(rename = "digestType")]
    pub r#digest_type: Option<i32>,
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Option<String>,
    #[serde(rename = "flags")]
    pub r#flags: Option<String>,
    #[serde(rename = "keyTag")]
    pub r#key_tag: Option<i32>,
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Option<i32>,
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Option<String>,
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Option<i32>,
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Option<f64>,
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Option<i32>,
    #[serde(rename = "longDirection")]
    pub r#long_direction: Option<String>,
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Option<i32>,
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Option<f64>,
    #[serde(rename = "matchingType")]
    pub r#matching_type: Option<i32>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Option<f64>,
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Option<f64>,
    #[serde(rename = "preference")]
    pub r#preference: Option<i32>,
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    #[serde(rename = "proto")]
    pub r#proto: Option<String>,
    #[serde(rename = "protocol")]
    pub r#protocol: Option<i32>,
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    #[serde(rename = "replacement")]
    pub r#replacement: Option<String>,
    #[serde(rename = "selector")]
    pub r#selector: Option<i32>,
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    #[serde(rename = "size")]
    pub r#size: Option<f64>,
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    #[serde(rename = "usage")]
    pub r#usage: Option<i32>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RulesetRule {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Option<crate::types::RulesetRuleActionParameters>,
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Option<crate::types::RulesetRuleExposedCredentialCheck>,
    #[serde(rename = "expression")]
    pub r#expression: String,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Option<String>,
    #[serde(rename = "logging")]
    pub r#logging: Option<crate::types::RulesetRuleLogging>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Option<crate::types::RulesetRuleRatelimit>,
    #[serde(rename = "ref")]
    pub r#ref: Option<String>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParameters {
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Option<Vec<i32>>,
    #[serde(rename = "algorithms")]
    pub r#algorithms: Option<Vec<crate::types::RulesetRuleActionParametersAlgorithm>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<bool>,
    #[serde(rename = "autominifies")]
    pub r#autominifies: Option<Vec<crate::types::RulesetRuleActionParametersAutominify>>,
    #[serde(rename = "bic")]
    pub r#bic: Option<bool>,
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Option<crate::types::RulesetRuleActionParametersBrowserTtl>,
    #[serde(rename = "cache")]
    pub r#cache: Option<bool>,
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Option<crate::types::RulesetRuleActionParametersCacheKey>,
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Option<Vec<String>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Option<bool>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Option<bool>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Option<bool>,
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Option<crate::types::RulesetRuleActionParametersEdgeTtl>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<bool>,
    #[serde(rename = "fromList")]
    pub r#from_list: Option<crate::types::RulesetRuleActionParametersFromList>,
    #[serde(rename = "fromValue")]
    pub r#from_value: Option<crate::types::RulesetRuleActionParametersFromValue>,
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<crate::types::RulesetRuleActionParametersHeader>>,
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<String>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "increment")]
    pub r#increment: Option<i32>,
    #[serde(rename = "matchedData")]
    pub r#matched_data: Option<crate::types::RulesetRuleActionParametersMatchedData>,
    #[serde(rename = "mirage")]
    pub r#mirage: Option<bool>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<bool>,
    #[serde(rename = "origin")]
    pub r#origin: Option<crate::types::RulesetRuleActionParametersOrigin>,
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Option<bool>,
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Option<bool>,
    #[serde(rename = "overrides")]
    pub r#overrides: Option<crate::types::RulesetRuleActionParametersOverrides>,
    #[serde(rename = "phases")]
    pub r#phases: Option<Vec<String>>,
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[serde(rename = "products")]
    pub r#products: Option<Vec<String>>,
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Option<i32>,
    #[serde(rename = "requestFields")]
    pub r#request_fields: Option<Vec<String>>,
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Option<bool>,
    #[serde(rename = "responseFields")]
    pub r#response_fields: Option<Vec<String>>,
    #[serde(rename = "responses")]
    pub r#responses: Option<Vec<crate::types::RulesetRuleActionParametersResponse>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<bool>,
    #[serde(rename = "rules")]
    pub r#rules: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ruleset")]
    pub r#ruleset: Option<String>,
    #[serde(rename = "rulesets")]
    pub r#rulesets: Option<Vec<String>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Option<crate::types::RulesetRuleActionParametersServeStale>,
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Option<bool>,
    #[serde(rename = "sni")]
    pub r#sni: Option<crate::types::RulesetRuleActionParametersSni>,
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "sxg")]
    pub r#sxg: Option<bool>,
    #[serde(rename = "uri")]
    pub r#uri: Option<crate::types::RulesetRuleActionParametersUri>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAlgorithm {
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAutominify {
    #[serde(rename = "css")]
    pub r#css: Option<bool>,
    #[serde(rename = "html")]
    pub r#html: Option<bool>,
    #[serde(rename = "js")]
    pub r#js: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersBrowserTtl {
    #[serde(rename = "default")]
    pub r#default: Option<i32>,
    #[serde(rename = "mode")]
    pub r#mode: String,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKey {
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<bool>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<bool>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKey>,
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie: Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>,
    #[serde(rename = "header")]
    pub r#header: Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>,
    #[serde(rename = "host")]
    pub r#host: Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>,
    #[serde(rename = "queryString")]
    pub r#query_string:
        Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>,
    #[serde(rename = "user")]
    pub r#user: Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Option<bool>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtl {
    #[serde(rename = "default")]
    pub r#default: Option<i32>,
    #[serde(rename = "mode")]
    pub r#mode: String,
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges:
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>,
    #[serde(rename = "value")]
    pub r#value: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    #[serde(rename = "from")]
    pub r#from: Option<i32>,
    #[serde(rename = "to")]
    pub r#to: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromList {
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValue {
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<bool>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Option<crate::types::RulesetRuleActionParametersFromValueTargetUrl>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValueTargetUrl {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersHeader {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "categories")]
    pub r#categories: Option<Vec<crate::types::RulesetRuleActionParametersOverridesCategory>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<crate::types::RulesetRuleActionParametersOverridesRule>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesCategory {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "category")]
    pub r#category: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesRule {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Option<i32>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersResponse {
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersSni {
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Option<bool>,
    #[serde(rename = "path")]
    pub r#path: Option<crate::types::RulesetRuleActionParametersUriPath>,
    #[serde(rename = "query")]
    pub r#query: Option<crate::types::RulesetRuleActionParametersUriQuery>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriPath {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriQuery {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleExposedCredentialCheck {
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Option<String>,
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleRatelimit {
    #[serde(rename = "characteristics")]
    pub r#characteristics: Option<Vec<String>>,
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Option<String>,
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Option<i32>,
    #[serde(rename = "period")]
    pub r#period: Option<i32>,
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Option<i32>,
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Option<bool>,
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Option<i32>,
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationDns {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationEdgeIps {
    #[serde(rename = "connectivity")]
    pub r#connectivity: Option<String>,
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginDns {
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginPortRange {
    #[serde(rename = "end")]
    pub r#end: i32,
    #[serde(rename = "start")]
    pub r#start: i32,
}

#[derive(serde::Serialize)]
pub struct SplitTunnelTunnel {
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "host")]
    pub r#host: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirus {
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: bool,
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: bool,
    #[serde(rename = "failClosed")]
    pub r#fail_closed: bool,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<crate::types::TeamsAccountAntivirusNotificationSettings>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirusNotificationSettings {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    #[serde(rename = "supportUrl")]
    pub r#support_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBlockPage {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Option<String>,
    #[serde(rename = "headerText")]
    pub r#header_text: Option<String>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Option<String>,
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Option<String>,
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBodyScanning {
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: String,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountExtendedEmailMatching {
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountFips {
    #[serde(rename = "tls")]
    pub r#tls: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLogging {
    #[serde(rename = "redactPii")]
    pub r#redact_pii: bool,
    #[serde(rename = "settingsByRuleType")]
    pub r#settings_by_rule_type: crate::types::TeamsAccountLoggingSettingsByRuleType,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    #[serde(rename = "dns")]
    pub r#dns: crate::types::TeamsAccountLoggingSettingsByRuleTypeDns,
    #[serde(rename = "http")]
    pub r#http: crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp,
    #[serde(rename = "l4")]
    pub r#l_4: crate::types::TeamsAccountLoggingSettingsByRuleTypeL4,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeDns {
    #[serde(rename = "logAll")]
    pub r#log_all: bool,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeHttp {
    #[serde(rename = "logAll")]
    pub r#log_all: bool,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeL4 {
    #[serde(rename = "logAll")]
    pub r#log_all: bool,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountPayloadLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: String,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountProxy {
    #[serde(rename = "rootCa")]
    pub r#root_ca: bool,
    #[serde(rename = "tcp")]
    pub r#tcp: bool,
    #[serde(rename = "udp")]
    pub r#udp: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountSshSessionLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: String,
}

#[derive(serde::Serialize)]
pub struct TeamsLocationNetwork {
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "network")]
    pub r#network: String,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettings {
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Option<bool>,
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Option<crate::types::TeamsRuleRuleSettingsAuditSsh>,
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Option<crate::types::TeamsRuleRuleSettingsBisoAdminControls>,
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Option<bool>,
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Option<String>,
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Option<bool>,
    #[serde(rename = "checkSession")]
    pub r#check_session: Option<crate::types::TeamsRuleRuleSettingsCheckSession>,
    #[serde(rename = "egress")]
    pub r#egress: Option<crate::types::TeamsRuleRuleSettingsEgress>,
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Option<bool>,
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Option<bool>,
    #[serde(rename = "l4override")]
    pub r#l_4_override: Option<crate::types::TeamsRuleRuleSettingsL4override>,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<crate::types::TeamsRuleRuleSettingsNotificationSettings>,
    #[serde(rename = "overrideHost")]
    pub r#override_host: Option<String>,
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Option<Vec<String>>,
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Option<crate::types::TeamsRuleRuleSettingsPayloadLog>,
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Option<crate::types::TeamsRuleRuleSettingsUntrustedCert>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    #[serde(rename = "commandLogging")]
    pub r#command_logging: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Option<bool>,
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Option<bool>,
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Option<bool>,
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Option<bool>,
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsCheckSession {
    #[serde(rename = "duration")]
    pub r#duration: String,
    #[serde(rename = "enforce")]
    pub r#enforce: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsEgress {
    #[serde(rename = "ipv4")]
    pub r#ipv_4: String,
    #[serde(rename = "ipv4Fallback")]
    pub r#ipv_4_fallback: Option<String>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: String,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsL4override {
    #[serde(rename = "ip")]
    pub r#ip: String,
    #[serde(rename = "port")]
    pub r#port: i32,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsNotificationSettings {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    #[serde(rename = "supportUrl")]
    pub r#support_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsUntrustedCert {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfig {
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Vec<crate::types::TunnelConfigConfigIngressRule>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Option<crate::types::TunnelConfigConfigOriginRequest>,
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Option<crate::types::TunnelConfigConfigWarpRouting>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRule {
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>,
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    #[serde(rename = "service")]
    pub r#service: String,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequest {
    #[serde(rename = "access")]
    pub r#access: Option<crate::types::TunnelConfigConfigIngressRuleOriginRequestAccess>,
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Option<bool>,
    #[serde(rename = "caPool")]
    pub r#ca_pool: Option<String>,
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Option<String>,
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Option<bool>,
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Option<bool>,
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Option<String>,
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<crate::types::TunnelConfigConfigIngressRuleOriginRequestIpRule>>,
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Option<i32>,
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Option<String>,
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Option<bool>,
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Option<bool>,
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Option<String>,
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Option<String>,
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Option<i32>,
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Option<String>,
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Option<String>,
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestAccess {
    #[serde(rename = "audTags")]
    pub r#aud_tags: Option<Vec<String>>,
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    #[serde(rename = "teamName")]
    pub r#team_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestIpRule {
    #[serde(rename = "allow")]
    pub r#allow: Option<bool>,
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<i32>>,
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequest {
    #[serde(rename = "access")]
    pub r#access: Option<crate::types::TunnelConfigConfigOriginRequestAccess>,
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Option<bool>,
    #[serde(rename = "caPool")]
    pub r#ca_pool: Option<String>,
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Option<String>,
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Option<bool>,
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Option<bool>,
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Option<String>,
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<crate::types::TunnelConfigConfigOriginRequestIpRule>>,
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Option<i32>,
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Option<String>,
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Option<bool>,
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Option<bool>,
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Option<String>,
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Option<String>,
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Option<i32>,
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Option<String>,
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Option<String>,
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestAccess {
    #[serde(rename = "audTags")]
    pub r#aud_tags: Option<Vec<String>>,
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    #[serde(rename = "teamName")]
    pub r#team_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestIpRule {
    #[serde(rename = "allow")]
    pub r#allow: Option<bool>,
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<i32>>,
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigWarpRouting {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct UserAgentBlockingRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: String,
    #[serde(rename = "value")]
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomAdditionalRoute {
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomRulesRule {
    #[serde(rename = "action")]
    pub r#action: String,
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "expression")]
    pub r#expression: String,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptAnalyticsEngineBinding {
    #[serde(rename = "dataset")]
    pub r#dataset: String,
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptD1DatabaseBinding {
    #[serde(rename = "databaseId")]
    pub r#database_id: String,
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptKvNamespaceBinding {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlacement {
    #[serde(rename = "mode")]
    pub r#mode: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlainTextBinding {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "text")]
    pub r#text: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptQueueBinding {
    #[serde(rename = "binding")]
    pub r#binding: String,
    #[serde(rename = "queue")]
    pub r#queue: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptR2BucketBinding {
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptSecretTextBinding {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "text")]
    pub r#text: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Option<String>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "service")]
    pub r#service: String,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptWebassemblyBinding {
    #[serde(rename = "module")]
    pub r#module: String,
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct ZoneLockdownConfiguration {
    #[serde(rename = "target")]
    pub r#target: String,
    #[serde(rename = "value")]
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSetting {
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Option<String>,
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Option<String>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<String>,
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Option<String>,
    #[serde(rename = "brotli")]
    pub r#brotli: Option<String>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Option<i32>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Option<String>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Option<String>,
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Option<i32>,
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Option<String>,
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Option<String>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<String>,
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Option<String>,
    #[serde(rename = "fonts")]
    pub r#fonts: Option<String>,
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Option<String>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<String>,
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    #[serde(rename = "http3")]
    pub r#http_3: Option<String>,
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Option<String>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Option<String>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Option<String>,
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Option<i32>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    #[serde(rename = "minify")]
    pub r#minify: Option<crate::types::ZoneSettingsOverrideInitialSettingMinify>,
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Option<crate::types::ZoneSettingsOverrideInitialSettingMobileRedirect>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<String>,
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Option<String>,
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Option<String>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Option<String>,
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Option<String>,
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Option<String>,
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Option<String>,
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Option<String>,
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Option<String>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Option<String>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<String>,
    #[serde(rename = "securityHeader")]
    pub r#security_header: Option<crate::types::ZoneSettingsOverrideInitialSettingSecurityHeader>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Option<String>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Option<String>,
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Option<String>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Option<String>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Option<String>,
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Option<String>,
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Option<String>,
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
    #[serde(rename = "webp")]
    pub r#webp: Option<String>,
    #[serde(rename = "websockets")]
    pub r#websockets: Option<String>,
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingMinify {
    #[serde(rename = "css")]
    pub r#css: String,
    #[serde(rename = "html")]
    pub r#html: String,
    #[serde(rename = "js")]
    pub r#js: String,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingMobileRedirect {
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: String,
    #[serde(rename = "status")]
    pub r#status: String,
    #[serde(rename = "stripUri")]
    pub r#strip_uri: bool,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideInitialSettingSecurityHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
    #[serde(rename = "nosniff")]
    pub r#nosniff: Option<bool>,
    #[serde(rename = "preload")]
    pub r#preload: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettings {
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Option<String>,
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Option<String>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<String>,
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Option<String>,
    #[serde(rename = "brotli")]
    pub r#brotli: Option<String>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Option<i32>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Option<String>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Option<String>,
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Option<i32>,
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Option<String>,
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Option<String>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<String>,
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Option<String>,
    #[serde(rename = "fonts")]
    pub r#fonts: Option<String>,
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Option<String>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<String>,
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    #[serde(rename = "http3")]
    pub r#http_3: Option<String>,
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Option<String>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Option<String>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Option<String>,
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Option<i32>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    #[serde(rename = "minify")]
    pub r#minify: Option<crate::types::ZoneSettingsOverrideSettingsMinify>,
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Option<crate::types::ZoneSettingsOverrideSettingsMobileRedirect>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<String>,
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Option<String>,
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Option<String>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Option<String>,
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Option<String>,
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Option<String>,
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Option<String>,
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Option<String>,
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Option<String>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Option<String>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<String>,
    #[serde(rename = "securityHeader")]
    pub r#security_header: Option<crate::types::ZoneSettingsOverrideSettingsSecurityHeader>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Option<String>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Option<String>,
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Option<String>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Option<String>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Option<String>,
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Option<String>,
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Option<String>,
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
    #[serde(rename = "webp")]
    pub r#webp: Option<String>,
    #[serde(rename = "websockets")]
    pub r#websockets: Option<String>,
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsMinify {
    #[serde(rename = "css")]
    pub r#css: String,
    #[serde(rename = "html")]
    pub r#html: String,
    #[serde(rename = "js")]
    pub r#js: String,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsMobileRedirect {
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: String,
    #[serde(rename = "status")]
    pub r#status: String,
    #[serde(rename = "stripUri")]
    pub r#strip_uri: bool,
}

#[derive(serde::Serialize)]
pub struct ZoneSettingsOverrideSettingsSecurityHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
    #[serde(rename = "nosniff")]
    pub r#nosniff: Option<bool>,
    #[serde(rename = "preload")]
    pub r#preload: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getAccountRolesRole {
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getAccountsAccount {
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getDevicePostureRulesRule {
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "expiration")]
    pub r#expiration: Option<String>,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(serde::Serialize)]
pub struct getDevicesDevice {
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    #[serde(rename = "deleted")]
    pub r#deleted: Option<bool>,
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<String>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Option<String>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Option<String>,
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Option<String>,
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Option<String>,
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Option<String>,
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    #[serde(rename = "updated")]
    pub r#updated: Option<String>,
    #[serde(rename = "userEmail")]
    pub r#user_email: Option<String>,
    #[serde(rename = "userId")]
    pub r#user_id: Option<String>,
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getDlpDatasetsDataset {
    #[serde(rename = "description")]
    pub r#description: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "secret")]
    pub r#secret: bool,
    #[serde(rename = "status")]
    pub r#status: String,
}

#[derive(serde::Serialize)]
pub struct getListsList {
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "numitems")]
    pub r#numitems: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct getLoadBalancerPoolsFilter {
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getLoadBalancerPoolsPool {
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Vec<String>,
    #[serde(rename = "createdOn")]
    pub r#created_on: String,
    #[serde(rename = "description")]
    pub r#description: String,
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "latitude")]
    pub r#latitude: f64,
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Vec<crate::types::getLoadBalancerPoolsPoolLoadShedding>,
    #[serde(rename = "longitude")]
    pub r#longitude: f64,
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: i32,
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: String,
    #[serde(rename = "monitor")]
    pub r#monitor: String,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: String,
    #[serde(rename = "origins")]
    pub r#origins: Vec<crate::types::getLoadBalancerPoolsPoolOrigin>,
}

#[derive(serde::Serialize)]
pub struct getLoadBalancerPoolsPoolLoadShedding {
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Option<f64>,
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Option<String>,
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Option<f64>,
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getLoadBalancerPoolsPoolOrigin {
    #[serde(rename = "address")]
    pub r#address: String,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<crate::types::getLoadBalancerPoolsPoolOriginHeader>>,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "weight")]
    pub r#weight: Option<f64>,
}

#[derive(serde::Serialize)]
pub struct getLoadBalancerPoolsPoolOriginHeader {
    #[serde(rename = "header")]
    pub r#header: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsFilter {
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "phase")]
    pub r#phase: Option<String>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRuleset {
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "kind")]
    pub r#kind: String,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "phase")]
    pub r#phase: String,
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<crate::types::getRulesetsRulesetRule>>,
    #[serde(rename = "version")]
    pub r#version: String,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRule {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Option<crate::types::getRulesetsRulesetRuleActionParameters>,
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check:
        Option<crate::types::getRulesetsRulesetRuleExposedCredentialCheck>,
    #[serde(rename = "expression")]
    pub r#expression: String,
    #[serde(rename = "id")]
    pub r#id: String,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Option<String>,
    #[serde(rename = "logging")]
    pub r#logging: Option<crate::types::getRulesetsRulesetRuleLogging>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Option<crate::types::getRulesetsRulesetRuleRatelimit>,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "version")]
    pub r#version: String,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParameters {
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Option<Vec<i32>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<bool>,
    #[serde(rename = "autominifies")]
    pub r#autominifies: Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersAutominify>>,
    #[serde(rename = "bic")]
    pub r#bic: Option<bool>,
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Option<crate::types::getRulesetsRulesetRuleActionParametersBrowserTtl>,
    #[serde(rename = "cache")]
    pub r#cache: Option<bool>,
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKey>,
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Option<Vec<String>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Option<bool>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Option<bool>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Option<bool>,
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Option<crate::types::getRulesetsRulesetRuleActionParametersEdgeTtl>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<bool>,
    #[serde(rename = "fromList")]
    pub r#from_list: Option<crate::types::getRulesetsRulesetRuleActionParametersFromList>,
    #[serde(rename = "fromValue")]
    pub r#from_value: Option<crate::types::getRulesetsRulesetRuleActionParametersFromValue>,
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersHeader>>,
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<String>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "increment")]
    pub r#increment: Option<i32>,
    #[serde(rename = "matchedData")]
    pub r#matched_data: Option<crate::types::getRulesetsRulesetRuleActionParametersMatchedData>,
    #[serde(rename = "mirage")]
    pub r#mirage: Option<bool>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<bool>,
    #[serde(rename = "origin")]
    pub r#origin: Option<crate::types::getRulesetsRulesetRuleActionParametersOrigin>,
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Option<bool>,
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Option<bool>,
    #[serde(rename = "overrides")]
    pub r#overrides: Option<crate::types::getRulesetsRulesetRuleActionParametersOverrides>,
    #[serde(rename = "phases")]
    pub r#phases: Option<Vec<String>>,
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[serde(rename = "products")]
    pub r#products: Option<Vec<String>>,
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Option<i32>,
    #[serde(rename = "requestFields")]
    pub r#request_fields: Option<Vec<String>>,
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Option<bool>,
    #[serde(rename = "responseFields")]
    pub r#response_fields: Option<Vec<String>>,
    #[serde(rename = "responses")]
    pub r#responses: Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersResponse>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<bool>,
    #[serde(rename = "rules")]
    pub r#rules: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ruleset")]
    pub r#ruleset: Option<String>,
    #[serde(rename = "rulesets")]
    pub r#rulesets: Option<Vec<String>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Option<crate::types::getRulesetsRulesetRuleActionParametersServeStale>,
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Option<bool>,
    #[serde(rename = "sni")]
    pub r#sni: Option<crate::types::getRulesetsRulesetRuleActionParametersSni>,
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "sxg")]
    pub r#sxg: Option<bool>,
    #[serde(rename = "uri")]
    pub r#uri: Option<crate::types::getRulesetsRulesetRuleActionParametersUri>,
    #[serde(rename = "version")]
    pub r#version: String,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersAutominify {
    #[serde(rename = "css")]
    pub r#css: Option<bool>,
    #[serde(rename = "html")]
    pub r#html: Option<bool>,
    #[serde(rename = "js")]
    pub r#js: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersBrowserTtl {
    #[serde(rename = "default")]
    pub r#default: Option<i32>,
    #[serde(rename = "mode")]
    pub r#mode: String,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKey {
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<bool>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<bool>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKey>,
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie:
        Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>,
    #[serde(rename = "header")]
    pub r#header:
        Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>,
    #[serde(rename = "host")]
    pub r#host: Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>,
    #[serde(rename = "queryString")]
    pub r#query_string:
        Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>,
    #[serde(rename = "user")]
    pub r#user: Option<crate::types::getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Option<bool>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersEdgeTtl {
    #[serde(rename = "default")]
    pub r#default: Option<i32>,
    #[serde(rename = "mode")]
    pub r#mode: String,
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Option<
        Vec<
            crate::types::getRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange,
        >,
    >,
    #[serde(rename = "value")]
    pub r#value: i32,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    #[serde(rename = "from")]
    pub r#from: Option<i32>,
    #[serde(rename = "to")]
    pub r#to: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersFromList {
    #[serde(rename = "key")]
    pub r#key: String,
    #[serde(rename = "name")]
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersFromValue {
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<bool>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    #[serde(rename = "targetUrl")]
    pub r#target_url:
        Option<crate::types::getRulesetsRulesetRuleActionParametersFromValueTargetUrl>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersFromValueTargetUrl {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersHeader {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "categories")]
    pub r#categories:
        Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersOverridesCategory>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<crate::types::getRulesetsRulesetRuleActionParametersOverridesRule>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Option<String>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersOverridesCategory {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "category")]
    pub r#category: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersOverridesRule {
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Option<i32>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Option<String>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersResponse {
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersSni {
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Option<bool>,
    #[serde(rename = "path")]
    pub r#path: Option<crate::types::getRulesetsRulesetRuleActionParametersUriPath>,
    #[serde(rename = "query")]
    pub r#query: Option<crate::types::getRulesetsRulesetRuleActionParametersUriQuery>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersUriPath {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleActionParametersUriQuery {
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleExposedCredentialCheck {
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Option<String>,
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getRulesetsRulesetRuleRatelimit {
    #[serde(rename = "characteristics")]
    pub r#characteristics: Option<Vec<String>>,
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Option<String>,
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Option<i32>,
    #[serde(rename = "period")]
    pub r#period: Option<i32>,
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Option<i32>,
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Option<bool>,
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Option<i32>,
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getZonesFilter {
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Option<String>,
    #[serde(rename = "match")]
    pub r#match: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "paused")]
    pub r#paused: Option<bool>,
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct getZonesZone {
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
