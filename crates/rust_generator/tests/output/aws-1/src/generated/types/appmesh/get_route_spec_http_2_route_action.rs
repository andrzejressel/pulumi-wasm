#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRouteSpecHttp2RouteAction {
    #[builder(into)]
    #[serde(rename = "weightedTargets")]
    pub r#weighted_targets: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteActionWeightedTarget>>,
}
