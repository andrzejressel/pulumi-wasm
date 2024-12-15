#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountLoggingSettingsByRuleTypeDns {
    /// Whether to log all activity.
    #[builder(into)]
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[builder(into)]
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}
