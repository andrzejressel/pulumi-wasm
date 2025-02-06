#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigAuxiliaryNodeGroupNodeGroup {
    /// The Node group resource name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The node group instance group configuration.
    #[builder(into, default)]
    #[serde(rename = "nodeGroupConfig")]
    pub r#node_group_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig>>,
    /// Node group roles. 
    /// One of `"DRIVER"`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Box<Vec<String>>,
}
