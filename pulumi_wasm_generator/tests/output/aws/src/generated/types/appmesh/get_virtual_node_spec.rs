#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpec {
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefault>>,
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackend>>,
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListener>>,
    #[builder(into)]
    #[serde(rename = "loggings")]
    pub r#loggings: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecLogging>>,
    #[builder(into)]
    #[serde(rename = "serviceDiscoveries")]
    pub r#service_discoveries: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscovery>>,
}