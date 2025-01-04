#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualRouterSpecListener {
    /// Port mapping information for the listener.
    #[builder(into)]
    #[serde(rename = "portMapping")]
    pub r#port_mapping: Box<super::super::types::appmesh::VirtualRouterSpecListenerPortMapping>,
}
