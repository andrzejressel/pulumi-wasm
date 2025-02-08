#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterOutpostConfig {
    /// The Amazon EC2 instance type for all Kubernetes control plane instances.
    #[builder(into)]
    #[serde(rename = "controlPlaneInstanceType")]
    pub r#control_plane_instance_type: Box<String>,
    /// An object representing the placement configuration for all the control plane instances of your local Amazon EKS cluster on AWS Outpost.
    #[builder(into)]
    #[serde(rename = "controlPlanePlacements")]
    pub r#control_plane_placements: Box<Vec<super::super::types::eks::GetClusterOutpostConfigControlPlanePlacement>>,
    /// List of ARNs of the Outposts hosting the EKS cluster. Only a single ARN is supported currently.
    #[builder(into)]
    #[serde(rename = "outpostArns")]
    pub r#outpost_arns: Box<Vec<String>>,
}
