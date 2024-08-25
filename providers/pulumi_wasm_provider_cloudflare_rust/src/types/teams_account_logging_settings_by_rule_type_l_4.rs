#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsAccountLoggingSettingsByRuleTypeL4 {
    /// Whether to log all activity.
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}
