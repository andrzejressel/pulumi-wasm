#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction {
    #[builder(into)]
    #[serde(rename = "eventAction")]
    pub r#event_action: Box<String>,
    /// Whether to send a notification.
    #[builder(into)]
    #[serde(rename = "notify")]
    pub r#notify: Box<bool>,
}