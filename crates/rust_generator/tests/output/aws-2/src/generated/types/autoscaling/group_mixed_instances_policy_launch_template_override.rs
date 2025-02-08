#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GroupMixedInstancesPolicyLaunchTemplateOverride {
    /// Override the instance type in the Launch Template with instance types that satisfy the requirements.
    #[builder(into, default)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements>>,
    /// Override the instance type in the Launch Template.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Override the instance launch template specification in the Launch Template.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateSpecification")]
    pub r#launch_template_specification: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification>>,
    /// Number of capacity units, which gives the instance type a proportional weight to other instance types.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<String>>,
}
