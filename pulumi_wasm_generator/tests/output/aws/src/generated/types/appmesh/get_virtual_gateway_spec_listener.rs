#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecListener {
    #[builder(into)]
    #[serde(rename = "connectionPools")]
    pub r#connection_pools: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPool>>,
    #[builder(into)]
    #[serde(rename = "healthChecks")]
    pub r#health_checks: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerHealthCheck>>,
    #[builder(into)]
    #[serde(rename = "portMappings")]
    pub r#port_mappings: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerPortMapping>>,
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerTl>>,
}
