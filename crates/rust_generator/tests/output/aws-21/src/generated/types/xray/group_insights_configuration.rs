#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupInsightsConfiguration {
    /// Specifies whether insights are enabled.
    #[builder(into)]
    #[serde(rename = "insightsEnabled")]
    pub r#insights_enabled: Box<bool>,
    /// Specifies whether insight notifications are enabled.
    #[builder(into, default)]
    #[serde(rename = "notificationsEnabled")]
    pub r#notifications_enabled: Box<Option<bool>>,
}
