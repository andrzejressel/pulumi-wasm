#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttpRouteActionRewrite {
    #[builder(into)]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteActionRewriteHostname>>,
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteActionRewritePath>>,
    #[builder(into)]
    #[serde(rename = "prefixes")]
    pub r#prefixes: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteActionRewritePrefix>>,
}
