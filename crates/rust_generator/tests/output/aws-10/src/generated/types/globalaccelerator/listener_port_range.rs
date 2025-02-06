#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerPortRange {
    /// The first port in the range of ports, inclusive.
    #[builder(into, default)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Box<Option<i32>>,
    /// The last port in the range of ports, inclusive.
    #[builder(into, default)]
    #[serde(rename = "toPort")]
    pub r#to_port: Box<Option<i32>>,
}
