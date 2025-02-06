#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecGrpcRouteAction {
    #[builder(into)]
    #[serde(rename = "weightedTargets")]
    pub r#weighted_targets: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteActionWeightedTarget>>,
}
