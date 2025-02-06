#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterCoreInstanceGroup {
    /// String containing the [EMR Auto Scaling Policy](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-automatic-scaling.html) JSON.
    #[builder(into, default)]
    #[serde(rename = "autoscalingPolicy")]
    pub r#autoscaling_policy: Box<Option<String>>,
    /// Bid price for each EC2 instance in the instance group, expressed in USD. By setting this attribute, the instance group is being declared as a Spot Instance, and will implicitly create a Spot request. Leave this blank to use On-Demand Instances.
    #[builder(into, default)]
    #[serde(rename = "bidPrice")]
    pub r#bid_price: Box<Option<String>>,
    /// Configuration block(s) for EBS volumes attached to each instance in the instance group. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ebsConfigs")]
    pub r#ebs_configs: Box<Option<Vec<super::super::types::emr::ClusterCoreInstanceGroupEbsConfig>>>,
    /// Core node type Instance Group ID, if using Instance Group for this node type.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Target number of instances for the instance group. Must be at least 1. Defaults to 1.
    #[builder(into, default)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<Option<i32>>,
    /// EC2 instance type for all instances in the instance group.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// Friendly name given to the instance group.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
