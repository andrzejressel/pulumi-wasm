#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttpRouteAction {
    /// Gateway route action to rewrite.
    #[builder(into, default)]
    #[serde(rename = "rewrite")]
    pub r#rewrite: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewrite>>,
    /// Target that traffic is routed to when a request matches the gateway route.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionTarget>,
}
