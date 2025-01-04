#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecBackendDefaultClientPolicy {
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTl>>,
}
