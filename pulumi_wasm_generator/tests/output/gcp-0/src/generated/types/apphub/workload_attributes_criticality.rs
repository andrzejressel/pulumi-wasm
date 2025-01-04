#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadAttributesCriticality {
    /// Criticality type.
    /// Possible values are: `MISSION_CRITICAL`, `HIGH`, `MEDIUM`, `LOW`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
