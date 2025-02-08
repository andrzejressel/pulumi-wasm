#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecServiceDiscovery {
    /// Any AWS Cloud Map information for the virtual node.
    #[builder(into, default)]
    #[serde(rename = "awsCloudMap")]
    pub r#aws_cloud_map: Box<Option<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryAwsCloudMap>>,
    /// DNS service name for the virtual node.
    #[builder(into, default)]
    #[serde(rename = "dns")]
    pub r#dns: Box<Option<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryDns>>,
}
