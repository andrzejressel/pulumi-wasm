#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualServiceSpecProvider {
    #[builder(into)]
    #[serde(rename = "virtualNodes")]
    pub r#virtual_nodes: Box<Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualNode>>,
    #[builder(into)]
    #[serde(rename = "virtualRouters")]
    pub r#virtual_routers: Box<Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualRouter>>,
}