#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualRouterSpecListener {
    #[builder(into)]
    #[serde(rename = "portMappings")]
    pub r#port_mappings: Box<Vec<super::super::types::appmesh::GetVirtualRouterSpecListenerPortMapping>>,
}
