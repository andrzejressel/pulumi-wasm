#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecBackendDefaultClientPolicyTl {
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate>>,
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<i32>>,
    #[builder(into)]
    #[serde(rename = "validations")]
    pub r#validations: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidation>>,
}