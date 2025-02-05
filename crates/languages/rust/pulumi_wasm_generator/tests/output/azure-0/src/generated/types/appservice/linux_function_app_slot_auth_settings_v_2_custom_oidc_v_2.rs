#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppSlotAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into, default)]
    #[serde(rename = "authorisationEndpoint")]
    pub r#authorisation_endpoint: Box<Option<String>>,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into, default)]
    #[serde(rename = "certificationUri")]
    pub r#certification_uri: Box<Option<String>>,
    /// The Client Credential Method used.
    #[builder(into, default)]
    #[serde(rename = "clientCredentialMethod")]
    pub r#client_credential_method: Box<Option<String>>,
    /// The ID of the Client to use to authenticate with the Custom OIDC.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The App Setting name that contains the secret for this Custom OIDC Client. This is generated from `name` above and suffixed with `_PROVIDER_AUTHENTICATION_SECRET`.
    #[builder(into, default)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<Option<String>>,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into, default)]
    #[serde(rename = "issuerEndpoint")]
    pub r#issuer_endpoint: Box<Option<String>>,
    /// The name of the Custom OIDC Authentication Provider.
    /// 
    /// > **NOTE:** An `app_setting` matching this value in upper case with the suffix of `_PROVIDER_AUTHENTICATION_SECRET` is required. e.g. `MYOIDC_PROVIDER_AUTHENTICATION_SECRET` for a value of `myoidc`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the claim that contains the users name.
    #[builder(into, default)]
    #[serde(rename = "nameClaimType")]
    pub r#name_claim_type: Box<Option<String>>,
    /// The app setting name that contains the `client_secret` value used for the Custom OIDC Login.
    #[builder(into)]
    #[serde(rename = "openidConfigurationEndpoint")]
    pub r#openid_configuration_endpoint: Box<String>,
    /// The list of the scopes that should be requested while authenticating.
    #[builder(into, default)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into, default)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<Option<String>>,
}
