#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerTlValidation {
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationSubjectAlternativeName>>,
    #[builder(into)]
    #[serde(rename = "trusts")]
    pub r#trusts: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTlValidationTrust>>,
}
