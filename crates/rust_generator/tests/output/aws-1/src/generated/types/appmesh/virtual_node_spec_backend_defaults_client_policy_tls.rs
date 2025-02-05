#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecBackendDefaultsClientPolicyTls {
    /// Listener's TLS certificate.
    #[builder(into, default)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate>>,
    /// Whether the policy is enforced. Default is `true`.
    #[builder(into, default)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<Option<bool>>,
    /// One or more ports that the policy is enforced for.
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// Listener's Transport Layer Security (TLS) validation context.
    #[builder(into)]
    #[serde(rename = "validation")]
    pub r#validation: Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation>,
}
