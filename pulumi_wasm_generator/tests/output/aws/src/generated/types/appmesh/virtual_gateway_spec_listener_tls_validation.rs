#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecListenerTlsValidation {
    /// SANs for a virtual gateway's listener's Transport Layer Security (TLS) validation context.
    #[builder(into, default)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Option<super::super::types::appmesh::VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames>>,
    /// TLS validation context trust.
    #[builder(into)]
    #[serde(rename = "trust")]
    pub r#trust: Box<super::super::types::appmesh::VirtualGatewaySpecListenerTlsValidationTrust>,
}