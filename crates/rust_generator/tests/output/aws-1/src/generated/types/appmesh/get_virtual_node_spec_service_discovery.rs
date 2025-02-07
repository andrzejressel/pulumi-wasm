#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecServiceDiscovery {
    #[builder(into)]
    #[serde(rename = "awsCloudMaps")]
    pub r#aws_cloud_maps: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryAwsCloudMap>>,
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryDn>>,
}
