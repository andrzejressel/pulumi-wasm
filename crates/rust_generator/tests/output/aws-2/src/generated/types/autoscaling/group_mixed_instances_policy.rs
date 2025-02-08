#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GroupMixedInstancesPolicy {
    /// Nested argument containing settings on how to mix on-demand and Spot instances in the Auto Scaling group. Defined below.
    #[builder(into, default)]
    #[serde(rename = "instancesDistribution")]
    pub r#instances_distribution: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyInstancesDistribution>>,
    /// Nested argument containing launch template settings along with the overrides to specify multiple instance types and weights. Defined below.
    #[builder(into)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplate>,
}
