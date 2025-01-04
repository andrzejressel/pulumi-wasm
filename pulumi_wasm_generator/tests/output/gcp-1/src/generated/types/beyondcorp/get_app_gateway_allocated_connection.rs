#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppGatewayAllocatedConnection {
    /// The ingress port of an allocated connection.
    #[builder(into)]
    #[serde(rename = "ingressPort")]
    pub r#ingress_port: Box<i32>,
    /// The PSC uri of an allocated connection.
    #[builder(into)]
    #[serde(rename = "pscUri")]
    pub r#psc_uri: Box<String>,
}
