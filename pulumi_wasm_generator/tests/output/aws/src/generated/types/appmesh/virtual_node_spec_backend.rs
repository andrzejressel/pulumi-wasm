#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecBackend {
    /// Virtual service to use as a backend for a virtual node.
    #[builder(into)]
    #[serde(rename = "virtualService")]
    pub r#virtual_service: Box<super::super::types::appmesh::VirtualNodeSpecBackendVirtualService>,
}