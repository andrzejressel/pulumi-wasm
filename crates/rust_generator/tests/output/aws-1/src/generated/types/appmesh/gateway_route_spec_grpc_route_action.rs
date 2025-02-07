#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecGrpcRouteAction {
    /// Target that traffic is routed to when a request matches the gateway route.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<super::super::types::appmesh::GatewayRouteSpecGrpcRouteActionTarget>,
}
