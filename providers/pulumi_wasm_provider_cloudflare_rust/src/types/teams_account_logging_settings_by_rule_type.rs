#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    /// Logging configuration for DNS requests.
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    /// Logging configuration for HTTP requests.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    /// Logging configuration for layer 4 requests.
    #[builder(into)]
    #[serde(rename = "l4")]
    pub r#l_4: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}
