#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendAddressPoolTunnelInterface {
    /// The unique identifier of this Gateway Load Balancer Tunnel Interface.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<i32>,
    /// The port number that this Gateway Load Balancer Tunnel Interface listens to.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The protocol used for this Gateway Load Balancer Tunnel Interface. Possible values are `None`, `Native` and `VXLAN`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The traffic type of this Gateway Load Balancer Tunnel Interface. Possible values are `None`, `Internal` and `External`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}