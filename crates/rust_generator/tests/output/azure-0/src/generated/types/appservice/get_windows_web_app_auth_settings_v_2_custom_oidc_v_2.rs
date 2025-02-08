#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsWebAppAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "authorisationEndpoint")]
    pub r#authorisation_endpoint: Box<String>,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "certificationUri")]
    pub r#certification_uri: Box<String>,
    /// The Client Credential Method used.
    #[builder(into)]
    #[serde(rename = "clientCredentialMethod")]
    pub r#client_credential_method: Box<String>,
    /// The OAuth 2.0 client ID used by the app for authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The app setting name containing the OAuth 2.0 client secret used by the app for authentication.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<String>,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "issuerEndpoint")]
    pub r#issuer_endpoint: Box<String>,
    /// The name of this Windows Web App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the claim that contains the users name.
    #[builder(into)]
    #[serde(rename = "nameClaimType")]
    pub r#name_claim_type: Box<String>,
    /// The endpoint used for OpenID Connect Discovery. For example `https://example.com/.well-known/openid-configuration`.
    #[builder(into)]
    #[serde(rename = "openidConfigurationEndpoint")]
    pub r#openid_configuration_endpoint: Box<String>,
    /// The list of the scopes that are requested while authenticating.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Vec<String>>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<String>,
}
