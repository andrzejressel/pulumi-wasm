#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification {
    /// ID of the launch template. Conflicts with `launch_template_name`.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<Option<String>>,
    /// Name of the launch template. Conflicts with `launch_template_id`.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
