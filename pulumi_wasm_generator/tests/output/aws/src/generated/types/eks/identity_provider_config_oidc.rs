#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IdentityProviderConfigOidc {
    /// Client ID for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The JWT claim that the provider will use to return groups.
    #[builder(into, default)]
    #[serde(rename = "groupsClaim")]
    pub r#groups_claim: Box<Option<String>>,
    /// A prefix that is prepended to group claims e.g., `oidc:`.
    #[builder(into, default)]
    #[serde(rename = "groupsPrefix")]
    pub r#groups_prefix: Box<Option<String>>,
    /// The name of the identity provider config.
    #[builder(into)]
    #[serde(rename = "identityProviderConfigName")]
    pub r#identity_provider_config_name: Box<String>,
    /// Issuer URL for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Box<String>,
    /// The key value pairs that describe required claims in the identity token.
    #[builder(into, default)]
    #[serde(rename = "requiredClaims")]
    pub r#required_claims: Box<Option<std::collections::HashMap<String, String>>>,
    /// The JWT claim that the provider will use as the username.
    #[builder(into, default)]
    #[serde(rename = "usernameClaim")]
    pub r#username_claim: Box<Option<String>>,
    /// A prefix that is prepended to username claims.
    #[builder(into, default)]
    #[serde(rename = "usernamePrefix")]
    pub r#username_prefix: Box<Option<String>>,
}
