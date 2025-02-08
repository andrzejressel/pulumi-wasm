#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualServiceSpecProviderVirtualNode {
    /// Name of the virtual node that is acting as a service provider. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualNodeName")]
    pub r#virtual_node_name: Box<String>,
}
