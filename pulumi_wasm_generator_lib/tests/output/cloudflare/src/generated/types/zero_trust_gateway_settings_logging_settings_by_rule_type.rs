#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[builder(into)]
    #[serde(rename = "l4")]
    pub r#l_4: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4>,
}