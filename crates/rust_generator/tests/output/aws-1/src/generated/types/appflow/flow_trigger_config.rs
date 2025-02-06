#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowTriggerConfig {
    /// Configuration details of a schedule-triggered flow as defined by the user. Currently, these settings only apply to the `Scheduled` trigger type. See Scheduled Trigger Properties for details.
    #[builder(into, default)]
    #[serde(rename = "triggerProperties")]
    pub r#trigger_properties: Box<Option<super::super::types::appflow::FlowTriggerConfigTriggerProperties>>,
    /// Type of flow trigger. Valid values are `Scheduled`, `Event`, and `OnDemand`.
    #[builder(into)]
    #[serde(rename = "triggerType")]
    pub r#trigger_type: Box<String>,
}
