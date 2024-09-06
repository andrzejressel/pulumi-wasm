#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[serde(rename = "dns")]
    pub r#dns: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[serde(rename = "http")]
    pub r#http: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[serde(rename = "l4")]
    pub r#l_4: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}
