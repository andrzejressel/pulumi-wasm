#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNodeGroupResource {
    /// List of objects containing information about AutoScaling Groups.
    #[builder(into)]
    #[serde(rename = "autoscalingGroups")]
    pub r#autoscaling_groups: Box<Vec<super::super::types::eks::GetNodeGroupResourceAutoscalingGroup>>,
    /// Identifier of the remote access EC2 Security Group.
    #[builder(into)]
    #[serde(rename = "remoteAccessSecurityGroupId")]
    pub r#remote_access_security_group_id: Box<String>,
}
