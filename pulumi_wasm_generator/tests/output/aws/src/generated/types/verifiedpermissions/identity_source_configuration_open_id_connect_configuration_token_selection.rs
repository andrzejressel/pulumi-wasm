#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection {
    /// The OIDC configuration for processing access tokens. See Access Token Only below.
    #[builder(into, default)]
    #[serde(rename = "accessTokenOnly")]
    pub r#access_token_only: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly>>,
    /// The OIDC configuration for processing identity (ID) tokens. See Identity Token Only below.
    #[builder(into, default)]
    #[serde(rename = "identityTokenOnly")]
    pub r#identity_token_only: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly>>,
}
