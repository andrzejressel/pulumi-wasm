#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualNodeSpec {
    /// Defaults for backends.
    #[builder(into, default)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Box<Option<super::super::types::appmesh::VirtualNodeSpecBackendDefaults>>,
    /// Backends to which the virtual node is expected to send outbound traffic.
    #[builder(into, default)]
    #[serde(rename = "backends")]
    pub r#backends: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecBackend>>>,
    /// Listeners from which the virtual node is expected to receive inbound traffic.
    #[builder(into, default)]
    #[serde(rename = "listeners")]
    pub r#listeners: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecListener>>>,
    /// Inbound and outbound access logging information for the virtual node.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<super::super::types::appmesh::VirtualNodeSpecLogging>>,
    /// Service discovery information for the virtual node.
    #[builder(into, default)]
    #[serde(rename = "serviceDiscovery")]
    pub r#service_discovery: Box<Option<super::super::types::appmesh::VirtualNodeSpecServiceDiscovery>>,
}
