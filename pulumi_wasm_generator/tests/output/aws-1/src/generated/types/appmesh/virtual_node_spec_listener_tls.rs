#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerTls {
    /// Listener's TLS certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<super::super::types::appmesh::VirtualNodeSpecListenerTlsCertificate>,
    /// Listener's TLS mode. Valid values: `DISABLED`, `PERMISSIVE`, `STRICT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Listener's Transport Layer Security (TLS) validation context.
    #[builder(into, default)]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerTlsValidation>>,
}
