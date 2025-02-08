#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfiguration {
    /// Specifies the configuration details of an Amazon Cognito user pool that Verified Permissions can use as a source of authenticated identities as entities. See Cognito User Pool Configuration below.
    #[builder(into, default)]
    #[serde(rename = "cognitoUserPoolConfiguration")]
    pub r#cognito_user_pool_configuration: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationCognitoUserPoolConfiguration>>,
    /// Specifies the configuration details of an OpenID Connect (OIDC) identity provider, or identity source, that Verified Permissions can use to generate entities from authenticated identities. See Open ID Connect Configuration below.
    #[builder(into, default)]
    #[serde(rename = "openIdConnectConfiguration")]
    pub r#open_id_connect_configuration: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfiguration>>,
}
