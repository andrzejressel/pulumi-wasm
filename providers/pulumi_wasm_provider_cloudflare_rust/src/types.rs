#[derive(serde::Serialize)]
pub struct AccessApplicationCorsHeader {
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Box<Option<bool>>,
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Box<Option<bool>>,
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Box<Option<bool>>,
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationFooterLink {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationLandingPageDesign {
    #[serde(rename = "buttonColor")]
    pub r#button_color: Box<Option<String>>,
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Box<Option<String>>,
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasApp {
    #[serde(rename = "appLauncherUrl")]
    pub r#app_launcher_url: Box<Option<String>>,
    #[serde(rename = "authType")]
    pub r#auth_type: Box<Option<String>>,
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    #[serde(rename = "consumerServiceUrl")]
    pub r#consumer_service_url: Box<Option<String>>,
    #[serde(rename = "customAttributes")]
    pub r#custom_attributes:
        Box<Option<Vec<crate::types::AccessApplicationSaasAppCustomAttribute>>>,
    #[serde(rename = "defaultRelayState")]
    pub r#default_relay_state: Box<Option<String>>,
    #[serde(rename = "grantTypes")]
    pub r#grant_types: Box<Option<Vec<String>>>,
    #[serde(rename = "groupFilterRegex")]
    pub r#group_filter_regex: Box<Option<String>>,
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: Box<Option<String>>,
    #[serde(rename = "nameIdFormat")]
    pub r#name_id_format: Box<Option<String>>,
    #[serde(rename = "nameIdTransformJsonata")]
    pub r#name_id_transform_jsonata: Box<Option<String>>,
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    #[serde(rename = "redirectUris")]
    pub r#redirect_uris: Box<Option<Vec<String>>>,
    #[serde(rename = "samlAttributeTransformJsonata")]
    pub r#saml_attribute_transform_jsonata: Box<Option<String>>,
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Box<Option<String>>,
    #[serde(rename = "ssoEndpoint")]
    pub r#sso_endpoint: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttribute {
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "nameFormat")]
    pub r#name_format: Box<Option<String>>,
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<crate::types::AccessApplicationSaasAppCustomAttributeSource>,
}

#[derive(serde::Serialize)]
pub struct AccessApplicationSaasAppCustomAttributeSource {
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupExcludeOkta {
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupIncludeOkta {
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessGroupRequireOkta {
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
    #[serde(rename = "chinaNetwork")]
    pub r#china_network: Box<Option<bool>>,
    #[serde(rename = "clientCertificateForwarding")]
    pub r#client_certificate_forwarding: Box<Option<bool>>,
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationCustomPage {
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessOrganizationLoginDesign {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    #[serde(rename = "textColor")]
    pub r#text_color: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyApprovalGroup {
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<i32>,
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyExcludeOkta {
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyIncludeOkta {
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
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
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
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessPolicyRequireOkta {
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
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct AccessRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AddressMapIp {
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}

#[derive(serde::Serialize)]
pub struct AddressMapMembership {
    #[serde(rename = "canDelete")]
    pub r#can_delete: Box<Option<bool>>,
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ApiShieldAuthIdCharacteristic {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenCondition {
    #[serde(rename = "requestIp")]
    pub r#request_ip: Box<Option<crate::types::ApiTokenConditionRequestIp>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenConditionRequestIp {
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ApiTokenPolicy {
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Box<Vec<String>>,
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
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Box<Option<String>>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<String>>,
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<crate::types::CustomHostnameSslSetting>>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Box<Option<Vec<crate::types::CustomHostnameSslValidationError>>>,
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Box<Option<Vec<crate::types::CustomHostnameSslValidationRecord>>>,
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct CustomHostnameSslSetting {
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
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
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    #[serde(rename = "geoRestrictions")]
    pub r#geo_restrictions: Box<Option<String>>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct CustomSslCustomSslPriority {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct DeviceDexTestData {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DeviceManagedNetworksConfig {
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<String>,
    #[serde(rename = "tlsSockaddr")]
    pub r#tls_sockaddr: Box<String>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureIntegrationConfig {
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Box<Option<String>>,
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Box<Option<String>>,
    #[serde(rename = "apiUrl")]
    pub r#api_url: Box<Option<String>>,
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    #[serde(rename = "customerId")]
    pub r#customer_id: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleInput {
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Box<Option<i32>>,
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Box<Option<String>>,
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Box<Option<Vec<String>>>,
    #[serde(rename = "cn")]
    pub r#cn: Box<Option<String>>,
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<Option<String>>,
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    #[serde(rename = "countOperator")]
    pub r#count_operator: Box<Option<String>>,
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "exists")]
    pub r#exists: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "infected")]
    pub r#infected: Box<Option<bool>>,
    #[serde(rename = "isActive")]
    pub r#is_active: Box<Option<bool>>,
    #[serde(rename = "issueCount")]
    pub r#issue_count: Box<Option<String>>,
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    #[serde(rename = "networkStatus")]
    pub r#network_status: Box<Option<String>>,
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    #[serde(rename = "os")]
    pub r#os: Box<Option<String>>,
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    #[serde(rename = "overall")]
    pub r#overall: Box<Option<String>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    #[serde(rename = "requireAll")]
    pub r#require_all: Box<Option<bool>>,
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<Option<String>>,
    #[serde(rename = "running")]
    pub r#running: Box<Option<bool>>,
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Box<Option<String>>,
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
    #[serde(rename = "totalScore")]
    pub r#total_score: Box<Option<i32>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DevicePostureRuleMatch {
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwareness {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "skip")]
    pub r#skip: Box<crate::types::DlpProfileContextAwarenessSkip>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileContextAwarenessSkip {
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntry {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<crate::types::DlpProfileEntryPattern>>,
}

#[derive(serde::Serialize)]
pub struct DlpProfileEntryPattern {
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllAction {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllMatcher {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleAction {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct EmailRoutingRuleMatcher {
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct FallbackDomainDomain {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct HealthcheckHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigCaching {
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct HyperdriveConfigOrigin {
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItem {
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<crate::types::ListItemValue>,
}

#[derive(serde::Serialize)]
pub struct ListItemHostname {
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItemRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<bool>>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<bool>>,
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
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ListItemValueRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<String>>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<String>>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<String>>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<String>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerCountryPool {
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerLocationStrategy {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerMonitorHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolLoadShedding {
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOrigin {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::LoadBalancerPoolOriginHeader>>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPoolOriginSteering {
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerPopPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRandomSteering {
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRegionPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRule {
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<crate::types::LoadBalancerRuleFixedResponse>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<crate::types::LoadBalancerRuleOverride>>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    #[serde(rename = "terminates")]
    pub r#terminates: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleFixedResponse {
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverride {
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideAdaptiveRouting>>>,
    #[serde(rename = "countryPools")]
    pub r#country_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideCountryPool>>>,
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Box<Option<Vec<String>>>,
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Box<Option<String>>,
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideLocationStrategy>>>,
    #[serde(rename = "popPools")]
    pub r#pop_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverridePopPool>>>,
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRandomSteering>>>,
    #[serde(rename = "regionPools")]
    pub r#region_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRegionPool>>>,
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<String>>,
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>>,
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Box<Option<i32>>,
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Box<Option<String>>,
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideCountryPool {
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideLocationStrategy {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverridePopPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRandomSteering {
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRegionPool {
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideSessionAffinityAttribute {
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LoadBalancerSessionAffinityAttribute {
    #[serde(rename = "drainDuration")]
    pub r#drain_duration: Box<Option<i32>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct LogpushJobOutputOptions {
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Box<Option<String>>,
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Box<Option<String>>,
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Box<Option<bool>>,
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Box<Option<String>>,
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Box<Option<String>>,
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Box<Option<String>>,
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Box<Option<String>>,
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedRequestHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedResponseHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyEmailIntegration {
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyFilters {
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    #[serde(rename = "affectedComponents")]
    pub r#affected_components: Box<Option<Vec<String>>>,
    #[serde(rename = "airportCodes")]
    pub r#airport_codes: Box<Option<Vec<String>>>,
    #[serde(rename = "alertTriggerPreferences")]
    pub r#alert_trigger_preferences: Box<Option<Vec<String>>>,
    #[serde(rename = "enableds")]
    pub r#enableds: Box<Option<Vec<String>>>,
    #[serde(rename = "environments")]
    pub r#environments: Box<Option<Vec<String>>>,
    #[serde(rename = "eventSources")]
    pub r#event_sources: Box<Option<Vec<String>>>,
    #[serde(rename = "eventTypes")]
    pub r#event_types: Box<Option<Vec<String>>>,
    #[serde(rename = "events")]
    pub r#events: Box<Option<Vec<String>>>,
    #[serde(rename = "groupBies")]
    pub r#group_bies: Box<Option<Vec<String>>>,
    #[serde(rename = "healthCheckIds")]
    pub r#health_check_ids: Box<Option<Vec<String>>>,
    #[serde(rename = "incidentImpacts")]
    pub r#incident_impacts: Box<Option<Vec<String>>>,
    #[serde(rename = "inputIds")]
    pub r#input_ids: Box<Option<Vec<String>>>,
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<Vec<String>>>,
    #[serde(rename = "megabitsPerSeconds")]
    pub r#megabits_per_seconds: Box<Option<Vec<String>>>,
    #[serde(rename = "newHealths")]
    pub r#new_healths: Box<Option<Vec<String>>>,
    #[serde(rename = "newStatuses")]
    pub r#new_statuses: Box<Option<Vec<String>>>,
    #[serde(rename = "packetsPerSeconds")]
    pub r#packets_per_seconds: Box<Option<Vec<String>>>,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Option<Vec<String>>>,
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    #[serde(rename = "projectIds")]
    pub r#project_ids: Box<Option<Vec<String>>>,
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<String>>>,
    #[serde(rename = "requestsPerSeconds")]
    pub r#requests_per_seconds: Box<Option<Vec<String>>>,
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Option<Vec<String>>>,
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<String>>>,
    #[serde(rename = "slos")]
    pub r#slos: Box<Option<Vec<String>>>,
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<String>>>,
    #[serde(rename = "targetHostnames")]
    pub r#target_hostnames: Box<Option<Vec<String>>>,
    #[serde(rename = "targetZoneNames")]
    pub r#target_zone_names: Box<Option<Vec<String>>>,
    #[serde(rename = "tunnelIds")]
    pub r#tunnel_ids: Box<Option<Vec<String>>>,
    #[serde(rename = "wheres")]
    pub r#wheres: Box<Option<Vec<String>>>,
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyPagerdutyIntegration {
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NotificationPolicyWebhooksIntegration {
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActions {
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<bool>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<String>>,
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    #[serde(rename = "bypassCacheOnCookie")]
    pub r#bypass_cache_on_cookie: Box<Option<String>>,
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<String>>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<String>>,
    #[serde(rename = "cacheKeyFields")]
    pub r#cache_key_fields: Box<Option<crate::types::PageRuleActionsCacheKeyFields>>,
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    #[serde(rename = "cacheOnCookie")]
    pub r#cache_on_cookie: Box<Option<String>>,
    #[serde(rename = "cacheTtlByStatuses")]
    pub r#cache_ttl_by_statuses: Box<Option<Vec<crate::types::PageRuleActionsCacheTtlByStatus>>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    #[serde(rename = "disablePerformance")]
    pub r#disable_performance: Box<Option<bool>>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    #[serde(rename = "disableSecurity")]
    pub r#disable_security: Box<Option<bool>>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    #[serde(rename = "edgeCacheTtl")]
    pub r#edge_cache_ttl: Box<Option<i32>>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    #[serde(rename = "explicitCacheControl")]
    pub r#explicit_cache_control: Box<Option<String>>,
    #[serde(rename = "forwardingUrl")]
    pub r#forwarding_url: Box<Option<crate::types::PageRuleActionsForwardingUrl>>,
    #[serde(rename = "hostHeaderOverride")]
    pub r#host_header_override: Box<Option<String>>,
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    #[serde(rename = "minifies")]
    pub r#minifies: Box<Option<Vec<crate::types::PageRuleActionsMinify>>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[serde(rename = "resolveOverride")]
    pub r#resolve_override: Box<Option<String>>,
    #[serde(rename = "respectStrongEtag")]
    pub r#respect_strong_etag: Box<Option<String>>,
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFields {
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>>,
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>>,
    #[serde(rename = "host")]
    pub r#host: Box<crate::types::PageRuleActionsCacheKeyFieldsHost>,
    #[serde(rename = "queryString")]
    pub r#query_string: Box<crate::types::PageRuleActionsCacheKeyFieldsQueryString>,
    #[serde(rename = "user")]
    pub r#user: Box<crate::types::PageRuleActionsCacheKeyFieldsUser>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "ignore")]
    pub r#ignore: Box<Option<bool>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheTtlByStatus {
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsForwardingUrl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PageRuleActionsMinify {
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectBuildConfig {
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigs {
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<crate::types::PagesProjectDeploymentConfigsPreview>>,
    #[serde(rename = "production")]
    pub r#production: Box<Option<crate::types::PagesProjectDeploymentConfigsProduction>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreview {
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsPreviewPlacement>>,
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>>,
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProduction {
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsProductionPlacement>>,
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsProductionServiceBinding>>>,
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSource {
    #[serde(rename = "config")]
    pub r#config: Box<Option<crate::types::PagesProjectSourceConfig>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct PagesProjectSourceConfig {
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Box<Option<bool>>,
    #[serde(rename = "owner")]
    pub r#owner: Box<Option<String>>,
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Box<Option<bool>>,
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Box<Option<Vec<String>>>,
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Box<Option<String>>,
    #[serde(rename = "productionBranch")]
    pub r#production_branch: Box<String>,
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Box<Option<bool>>,
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitAction {
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitActionResponse>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitActionResponse {
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RateLimitCorrelate {
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatch {
    #[serde(rename = "request")]
    pub r#request: Box<Option<crate::types::RateLimitMatchRequest>>,
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitMatchResponse>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchRequest {
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    #[serde(rename = "schemes")]
    pub r#schemes: Box<Option<Vec<String>>>,
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RateLimitMatchResponse {
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Box<Option<bool>>,
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
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::RulesetRuleActionParameters>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Box<Option<crate::types::RulesetRuleExposedCredentialCheck>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::RulesetRuleLogging>>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::RulesetRuleRatelimit>>,
    #[serde(rename = "ref")]
    pub r#ref: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParameters {
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    #[serde(rename = "algorithms")]
    pub r#algorithms: Box<Option<Vec<crate::types::RulesetRuleActionParametersAlgorithm>>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    #[serde(rename = "autominifies")]
    pub r#autominifies: Box<Option<Vec<crate::types::RulesetRuleActionParametersAutominify>>>,
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<crate::types::RulesetRuleActionParametersBrowserTtl>>,
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<crate::types::RulesetRuleActionParametersCacheKey>>,
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<crate::types::RulesetRuleActionParametersEdgeTtl>>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<crate::types::RulesetRuleActionParametersFromList>>,
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<crate::types::RulesetRuleActionParametersFromValue>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::RulesetRuleActionParametersHeader>>>,
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    #[serde(rename = "matchedData")]
    pub r#matched_data: Box<Option<crate::types::RulesetRuleActionParametersMatchedData>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<crate::types::RulesetRuleActionParametersOrigin>>,
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<crate::types::RulesetRuleActionParametersOverrides>>,
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<crate::types::RulesetRuleActionParametersResponse>>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<crate::types::RulesetRuleActionParametersServeStale>>,
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<crate::types::RulesetRuleActionParametersSni>>,
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<crate::types::RulesetRuleActionParametersUri>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAlgorithm {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAutominify {
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersBrowserTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKey {
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKey>>,
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    #[serde(rename = "host")]
    pub r#host: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    #[serde(rename = "queryString")]
    pub r#query_string:
        Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>,
    >,
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromList {
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValue {
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<crate::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValueTargetUrl {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersHeader {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesCategory>>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesRule>>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesCategory {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersResponse {
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersSni {
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::RulesetRuleActionParametersUriPath>>,
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::RulesetRuleActionParametersUriQuery>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriPath {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUriQuery {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleExposedCredentialCheck {
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct RulesetRuleRatelimit {
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationDns {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationEdgeIps {
    #[serde(rename = "connectivity")]
    pub r#connectivity: Box<Option<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginDns {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginPortRange {
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct SplitTunnelTunnel {
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirus {
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: Box<bool>,
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: Box<bool>,
    #[serde(rename = "failClosed")]
    pub r#fail_closed: Box<bool>,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsAccountAntivirusNotificationSettings>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirusNotificationSettings {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBlockPage {
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "footerText")]
    pub r#footer_text: Box<Option<String>>,
    #[serde(rename = "headerText")]
    pub r#header_text: Box<Option<String>>,
    #[serde(rename = "logoPath")]
    pub r#logo_path: Box<Option<String>>,
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Box<Option<String>>,
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountBodyScanning {
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountExtendedEmailMatching {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountFips {
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLogging {
    #[serde(rename = "redactPii")]
    pub r#redact_pii: Box<bool>,
    #[serde(rename = "settingsByRuleType")]
    pub r#settings_by_rule_type: Box<crate::types::TeamsAccountLoggingSettingsByRuleType>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    #[serde(rename = "dns")]
    pub r#dns: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    #[serde(rename = "http")]
    pub r#http: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    #[serde(rename = "l4")]
    pub r#l_4: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeDns {
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeHttp {
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeL4 {
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountPayloadLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountProxy {
    #[serde(rename = "rootCa")]
    pub r#root_ca: Box<bool>,
    #[serde(rename = "tcp")]
    pub r#tcp: Box<bool>,
    #[serde(rename = "udp")]
    pub r#udp: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsAccountSshSessionLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsLocationNetwork {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettings {
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Box<Option<bool>>,
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Box<Option<crate::types::TeamsRuleRuleSettingsAuditSsh>>,
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Box<Option<crate::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Box<Option<bool>>,
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Box<Option<String>>,
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Box<Option<bool>>,
    #[serde(rename = "checkSession")]
    pub r#check_session: Box<Option<crate::types::TeamsRuleRuleSettingsCheckSession>>,
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<crate::types::TeamsRuleRuleSettingsEgress>>,
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Box<Option<bool>>,
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Box<Option<bool>>,
    #[serde(rename = "l4override")]
    pub r#l_4_override: Box<Option<crate::types::TeamsRuleRuleSettingsL4Override>>,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsRuleRuleSettingsNotificationSettings>>,
    #[serde(rename = "overrideHost")]
    pub r#override_host: Box<Option<String>>,
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Box<Option<Vec<String>>>,
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Box<Option<crate::types::TeamsRuleRuleSettingsPayloadLog>>,
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Box<Option<crate::types::TeamsRuleRuleSettingsUntrustedCert>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsCheckSession {
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsEgress {
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<String>,
    #[serde(rename = "ipv4Fallback")]
    pub r#ipv_4_fallback: Box<Option<String>>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsL4Override {
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsNotificationSettings {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}

#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsUntrustedCert {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfig {
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Box<Vec<crate::types::TunnelConfigConfigIngressRule>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigOriginRequest>>,
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Box<Option<crate::types::TunnelConfigConfigWarpRouting>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRule {
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequest {
    #[serde(rename = "access")]
    pub r#access: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequestAccess>>,
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    #[serde(rename = "ipRules")]
    pub r#ip_rules:
        Box<Option<Vec<crate::types::TunnelConfigConfigIngressRuleOriginRequestIpRule>>>,
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestAccess {
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestIpRule {
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequest {
    #[serde(rename = "access")]
    pub r#access: Box<Option<crate::types::TunnelConfigConfigOriginRequestAccess>>,
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<crate::types::TunnelConfigConfigOriginRequestIpRule>>>,
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestAccess {
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigOriginRequestIpRule {
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct TunnelConfigConfigWarpRouting {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct UserAgentBlockingRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomAdditionalRoute {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct WaitingRoomRulesRule {
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptAnalyticsEngineBinding {
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptD1DatabaseBinding {
    #[serde(rename = "databaseId")]
    pub r#database_id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptKvNamespaceBinding {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptPlainTextBinding {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptQueueBinding {
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptR2BucketBinding {
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptSecretTextBinding {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

#[derive(serde::Serialize)]
pub struct WorkerScriptWebassemblyBinding {
    #[serde(rename = "module")]
    pub r#module: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ZoneLockdownConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
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
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetAccountsAccount {
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetDevicePostureRulesRule {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetDevicesDevice {
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    #[serde(rename = "deleted")]
    pub r#deleted: Box<Option<bool>>,
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<Option<String>>,
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    #[serde(rename = "updated")]
    pub r#updated: Box<Option<String>>,
    #[serde(rename = "userEmail")]
    pub r#user_email: Box<Option<String>>,
    #[serde(rename = "userId")]
    pub r#user_id: Box<Option<String>>,
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
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
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "numitems")]
    pub r#numitems: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsFilter {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPool {
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Box<Vec<String>>,
    #[serde(rename = "createdOn")]
    pub r#created_on: Box<String>,
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "latitude")]
    pub r#latitude: Box<f64>,
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Box<Vec<crate::types::GetLoadBalancerPoolsPoolLoadShedding>>,
    #[serde(rename = "longitude")]
    pub r#longitude: Box<f64>,
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: Box<i32>,
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: Box<String>,
    #[serde(rename = "monitor")]
    pub r#monitor: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: Box<String>,
    #[serde(rename = "origins")]
    pub r#origins: Box<Vec<crate::types::GetLoadBalancerPoolsPoolOrigin>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolOrigin {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::GetLoadBalancerPoolsPoolOriginHeader>>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}

#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolOriginHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsFilter {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "phase")]
    pub r#phase: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRuleset {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "phase")]
    pub r#phase: Box<String>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::GetRulesetsRulesetRule>>>,
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::GetRulesetsRulesetRuleActionParameters>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check:
        Box<Option<crate::types::GetRulesetsRulesetRuleExposedCredentialCheck>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::GetRulesetsRulesetRuleLogging>>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::GetRulesetsRulesetRuleRatelimit>>,
    #[serde(rename = "ref")]
    pub r#ref: Box<String>,
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParameters {
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    #[serde(rename = "autominifies")]
    pub r#autominifies:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersAutominify>>>,
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersBrowserTtl>>,
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKey>>,
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtl>>,
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromList>>,
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValue>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersHeader>>>,
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    #[serde(rename = "matchedData")]
    pub r#matched_data:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersMatchedData>>,
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOrigin>>,
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOverrides>>,
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersResponse>>>,
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersServeStale>>,
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersSni>>,
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUri>>,
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersAutominify {
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKey {
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    #[serde(rename = "customKey")]
    pub r#custom_key:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey>>,
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    #[serde(rename = "header")]
    pub r#header:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    #[serde(rename = "host")]
    pub r#host:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    #[serde(rename = "queryString")]
    pub r#query_string: Box<
        Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>,
    >,
    #[serde(rename = "user")]
    pub r#user:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost {
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromList {
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromValue {
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValueTargetUrl>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromValueTargetUrl {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersHeader {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "categories")]
    pub r#categories:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesCategory>>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesRule>>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesCategory {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersSni {
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriPath>>,
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriQuery>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUriPath {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUriQuery {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleExposedCredentialCheck {
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleRatelimit {
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetZonesFilter {
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Box<Option<String>>,
    #[serde(rename = "match")]
    pub r#match: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "paused")]
    pub r#paused: Box<Option<bool>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct GetZonesZone {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
