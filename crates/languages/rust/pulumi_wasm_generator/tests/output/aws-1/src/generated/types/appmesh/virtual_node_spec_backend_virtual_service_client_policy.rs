#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecBackendVirtualServiceClientPolicy {
    /// Transport Layer Security (TLS) client policy.
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<super::super::types::appmesh::VirtualNodeSpecBackendVirtualServiceClientPolicyTls>>,
}
