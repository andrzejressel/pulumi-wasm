#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMeshSpec {
    #[builder(into)]
    #[serde(rename = "egressFilters")]
    pub r#egress_filters: Box<Vec<super::super::types::appmesh::GetMeshSpecEgressFilter>>,
    #[builder(into)]
    #[serde(rename = "serviceDiscoveries")]
    pub r#service_discoveries: Box<Vec<super::super::types::appmesh::GetMeshSpecServiceDiscovery>>,
}