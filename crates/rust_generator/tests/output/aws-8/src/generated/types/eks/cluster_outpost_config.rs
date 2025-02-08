#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterOutpostConfig {
    /// The Amazon EC2 instance type that you want to use for your local Amazon EKS cluster on Outposts. The instance type that you specify is used for all Kubernetes control plane instances. The instance type can't be changed after cluster creation. Choose an instance type based on the number of nodes that your cluster will have. If your cluster will have:
    /// 
    /// * 1–20 nodes, then we recommend specifying a large instance type.
    /// 
    /// * 21–100 nodes, then we recommend specifying an xlarge instance type.
    /// 
    /// * 101–250 nodes, then we recommend specifying a 2xlarge instance type.
    /// 
    /// For a list of the available Amazon EC2 instance types, see Compute and storage in AWS Outposts rack features  The control plane is not automatically scaled by Amazon EKS.
    #[builder(into)]
    #[serde(rename = "controlPlaneInstanceType")]
    pub r#control_plane_instance_type: Box<String>,
    /// An object representing the placement configuration for all the control plane instances of your local Amazon EKS cluster on AWS Outpost.
    /// The `control_plane_placement` configuration block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "controlPlanePlacement")]
    pub r#control_plane_placement: Box<Option<super::super::types::eks::ClusterOutpostConfigControlPlanePlacement>>,
    /// The ARN of the Outpost that you want to use for your local Amazon EKS cluster on Outposts. This argument is a list of arns, but only a single Outpost ARN is supported currently.
    #[builder(into)]
    #[serde(rename = "outpostArns")]
    pub r#outpost_arns: Box<Vec<String>>,
}
