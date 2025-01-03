#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplate {
    /// List of overriding launch template specification objects.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecifications")]
    pub r#launch_template_specifications: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification>>,
    /// List of properties overriding the same properties in the launch template.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverride>>,
}
