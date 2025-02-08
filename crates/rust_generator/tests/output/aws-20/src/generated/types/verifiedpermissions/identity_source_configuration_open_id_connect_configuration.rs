#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IdentitySourceConfigurationOpenIdConnectConfiguration {
    /// A descriptive string that you want to prefix to user entities from your OIDC identity provider.
    #[builder(into, default)]
    #[serde(rename = "entityIdPrefix")]
    pub r#entity_id_prefix: Box<Option<String>>,
    /// The type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
    #[builder(into, default)]
    #[serde(rename = "groupConfiguration")]
    pub r#group_configuration: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration>>,
    /// The issuer URL of an OIDC identity provider. This URL must have an OIDC discovery endpoint at the path `.well-known/openid-configuration`.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// The token type that you want to process from your OIDC identity provider. Your policy store can process either identity (ID) or access tokens from a given OIDC identity source. See Token Selection below.
    #[builder(into, default)]
    #[serde(rename = "tokenSelection")]
    pub r#token_selection: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection>>,
}
