#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm {
    /// One or more ACM ARNs.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityArns")]
    pub r#certificate_authority_arns: Box<Vec<String>>,
}