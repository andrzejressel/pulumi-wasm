#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerNamedPort {
    /// The name of the port.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The port number.
    /// - - -
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
