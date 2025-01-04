#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduledQueryRulesAlertV2Action {
    /// List of Action Group resource IDs to invoke when the alert fires.
    #[builder(into, default)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Box<Option<Vec<String>>>,
    /// Specifies the properties of an alert payload.
    #[builder(into, default)]
    #[serde(rename = "customProperties")]
    pub r#custom_properties: Box<Option<std::collections::HashMap<String, String>>>,
}
