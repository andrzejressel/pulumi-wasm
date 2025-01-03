#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetLaunchTemplateConfigLaunchTemplateSpecification {
    /// The ID of the launch template.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<Option<String>>,
    /// The name of the launch template.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Box<Option<String>>,
    /// The launch template version number, `$Latest`, or `$Default.`
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
