#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpec {
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefault>>,
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListener>>,
    #[builder(into)]
    #[serde(rename = "loggings")]
    pub r#loggings: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecLogging>>,
}