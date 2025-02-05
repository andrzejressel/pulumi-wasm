#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition {
    /// Action setting that a log record must contain in order to meet the condition. Valid values for `action` are `ALLOW`, `BLOCK`, and `COUNT`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
}
