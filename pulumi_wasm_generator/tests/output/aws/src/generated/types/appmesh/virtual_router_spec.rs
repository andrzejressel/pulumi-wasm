#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualRouterSpec {
    /// Listeners that the virtual router is expected to receive inbound traffic from.
    /// Currently only one listener is supported per virtual router.
    #[builder(into, default)]
    #[serde(rename = "listeners")]
    pub r#listeners: Box<Option<Vec<super::super::types::appmesh::VirtualRouterSpecListener>>>,
}