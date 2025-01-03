#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerTlValidationTrust {
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationTrustFile>>,
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationTrustSd>>,
}
