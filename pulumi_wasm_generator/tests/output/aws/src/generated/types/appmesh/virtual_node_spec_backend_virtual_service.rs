#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecBackendVirtualService {
    /// Client policy for the backend.
    #[builder(into, default)]
    #[serde(rename = "clientPolicy")]
    pub r#client_policy: Box<Option<super::super::types::appmesh::VirtualNodeSpecBackendVirtualServiceClientPolicy>>,
    /// Name of the virtual service that is acting as a virtual node backend. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualServiceName")]
    pub r#virtual_service_name: Box<String>,
}
