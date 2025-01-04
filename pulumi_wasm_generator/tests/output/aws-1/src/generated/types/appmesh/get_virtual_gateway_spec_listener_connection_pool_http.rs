#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecListenerConnectionPoolHttp {
    #[builder(into)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<i32>,
    #[builder(into)]
    #[serde(rename = "maxPendingRequests")]
    pub r#max_pending_requests: Box<i32>,
}
