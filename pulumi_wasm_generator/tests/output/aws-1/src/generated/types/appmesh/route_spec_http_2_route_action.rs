#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecHttp2RouteAction {
    /// Targets that traffic is routed to when a request matches the route.
    /// You can specify one or more targets and their relative weights with which to distribute traffic.
    #[builder(into)]
    #[serde(rename = "weightedTargets")]
    pub r#weighted_targets: Box<Vec<super::super::types::appmesh::RouteSpecHttp2RouteActionWeightedTarget>>,
}
