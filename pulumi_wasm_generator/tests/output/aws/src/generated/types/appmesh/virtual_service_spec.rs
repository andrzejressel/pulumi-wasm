#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualServiceSpec {
    /// App Mesh object that is acting as the provider for a virtual service. You can specify a single virtual node or virtual router.
    #[builder(into, default)]
    #[serde(rename = "provider")]
    pub r#provider: Box<Option<super::super::types::appmesh::VirtualServiceSpecProvider>>,
}