#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ComputeEnvironmentComputeResourcesLaunchTemplate {
    /// ID of the launch template. You must specify either the launch template ID or launch template name in the request, but not both.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<Option<String>>,
    /// Name of the launch template.
    #[builder(into, default)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Box<Option<String>>,
    /// The version number of the launch template. Default: The default version of the launch template.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
