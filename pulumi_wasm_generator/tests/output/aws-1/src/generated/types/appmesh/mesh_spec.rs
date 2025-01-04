#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MeshSpec {
    /// Egress filter rules for the service mesh.
    #[builder(into, default)]
    #[serde(rename = "egressFilter")]
    pub r#egress_filter: Box<Option<super::super::types::appmesh::MeshSpecEgressFilter>>,
    /// The service discovery information for the service mesh.
    #[builder(into, default)]
    #[serde(rename = "serviceDiscovery")]
    pub r#service_discovery: Box<Option<super::super::types::appmesh::MeshSpecServiceDiscovery>>,
}
