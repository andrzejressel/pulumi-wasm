#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecGrpcRouteActionTarget {
    /// The port number that corresponds to the target for Virtual Service provider port. This is required when the provider (router or node) of the Virtual Service has multiple listeners.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Virtual service gateway route target.
    #[builder(into)]
    #[serde(rename = "virtualService")]
    pub r#virtual_service: Box<super::super::types::appmesh::GatewayRouteSpecGrpcRouteActionTargetVirtualService>,
}
