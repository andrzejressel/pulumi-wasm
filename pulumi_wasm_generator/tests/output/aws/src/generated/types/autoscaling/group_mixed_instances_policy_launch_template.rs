#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupMixedInstancesPolicyLaunchTemplate {
    /// Override the instance launch template specification in the Launch Template.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecification")]
    pub r#launch_template_specification: Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification>,
    /// List of nested arguments provides the ability to specify multiple instance types. This will override the same parameter in the launch template. For on-demand instances, Auto Scaling considers the order of preference of instance types to launch based on the order specified in the overrides list. Defined below.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverride>>>,
}
