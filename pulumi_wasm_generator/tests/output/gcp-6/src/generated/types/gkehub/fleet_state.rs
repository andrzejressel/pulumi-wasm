#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetState {
    /// (Output)
    /// Describes the state of a Fleet resource.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<String>>,
}
