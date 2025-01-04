#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecBackend {
    #[builder(into)]
    #[serde(rename = "virtualServices")]
    pub r#virtual_services: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualService>>,
}
