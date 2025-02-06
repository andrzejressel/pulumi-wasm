#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodeGroupResource {
    /// List of objects containing information about AutoScaling Groups.
    #[builder(into, default)]
    #[serde(rename = "autoscalingGroups")]
    pub r#autoscaling_groups: Box<Option<Vec<super::super::types::eks::NodeGroupResourceAutoscalingGroup>>>,
    /// Identifier of the remote access EC2 Security Group.
    #[builder(into, default)]
    #[serde(rename = "remoteAccessSecurityGroupId")]
    pub r#remote_access_security_group_id: Box<Option<String>>,
}
