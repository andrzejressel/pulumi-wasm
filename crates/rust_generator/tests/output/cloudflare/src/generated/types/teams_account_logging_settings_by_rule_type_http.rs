#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsAccountLoggingSettingsByRuleTypeHttp {
    /// Whether to log all activity.
    #[builder(into)]
    #[serde(rename = "logAll")]
    pub r#log_all: Box<bool>,
    #[builder(into)]
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: Box<bool>,
}
