#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification {
    /// ID of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<String>,
    /// Name of the launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Box<String>,
    /// Template version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
