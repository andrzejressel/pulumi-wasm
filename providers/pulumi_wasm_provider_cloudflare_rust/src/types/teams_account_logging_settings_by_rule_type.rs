#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleType {
    #[serde(rename = "dns")]
    pub r#dns: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeDns>,
    #[serde(rename = "http")]
    pub r#http: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeHttp>,
    #[serde(rename = "l4")]
    pub r#l_4: Box<crate::types::TeamsAccountLoggingSettingsByRuleTypeL4>,
}
