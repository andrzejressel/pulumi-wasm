#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AddonsConfigAddonsConfig {
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "advancedApiOpsConfig")]
    pub r#advanced_api_ops_config: Box<Option<super::super::types::apigee::AddonsConfigAddonsConfigAdvancedApiOpsConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "apiSecurityConfig")]
    pub r#api_security_config: Box<Option<super::super::types::apigee::AddonsConfigAddonsConfigApiSecurityConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "connectorsPlatformConfig")]
    pub r#connectors_platform_config: Box<Option<super::super::types::apigee::AddonsConfigAddonsConfigConnectorsPlatformConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "integrationConfig")]
    pub r#integration_config: Box<Option<super::super::types::apigee::AddonsConfigAddonsConfigIntegrationConfig>>,
    /// Configuration for the Monetization add-on.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "monetizationConfig")]
    pub r#monetization_config: Box<Option<super::super::types::apigee::AddonsConfigAddonsConfigMonetizationConfig>>,
}
