#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust {
    #[builder(into)]
    #[serde(rename = "acms")]
    pub r#acms: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm>>,
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile>>,
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd>>,
}