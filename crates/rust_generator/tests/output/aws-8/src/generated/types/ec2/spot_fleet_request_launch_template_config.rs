#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpotFleetRequestLaunchTemplateConfig {
    /// Launch template specification. See Launch Template Specification below for more details.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecification")]
    pub r#launch_template_specification: Box<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfigLaunchTemplateSpecification>,
    /// One or more override configurations. See Overrides below for more details.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfigOverride>>>,
}
