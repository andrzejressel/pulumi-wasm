#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualServiceSpecProvider {
    /// Virtual node associated with a virtual service.
    #[builder(into, default)]
    #[serde(rename = "virtualNode")]
    pub r#virtual_node: Box<Option<super::super::types::appmesh::VirtualServiceSpecProviderVirtualNode>>,
    /// Virtual router associated with a virtual service.
    #[builder(into, default)]
    #[serde(rename = "virtualRouter")]
    pub r#virtual_router: Box<Option<super::super::types::appmesh::VirtualServiceSpecProviderVirtualRouter>>,
}
