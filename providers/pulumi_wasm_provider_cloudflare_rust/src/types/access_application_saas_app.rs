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
