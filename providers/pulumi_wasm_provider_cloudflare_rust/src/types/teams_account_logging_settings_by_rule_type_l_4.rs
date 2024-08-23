#[derive(serde::Serialize)]
pub struct TeamsAccountLoggingSettingsByRuleTypeL4 {
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}
