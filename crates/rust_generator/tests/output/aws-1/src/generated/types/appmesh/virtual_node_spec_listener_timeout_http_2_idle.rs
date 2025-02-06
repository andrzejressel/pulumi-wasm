#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerTimeoutHttp2Idle {
    /// Unit of time. Valid values: `ms`, `s`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// Number of time units. Minimum value of `0`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
