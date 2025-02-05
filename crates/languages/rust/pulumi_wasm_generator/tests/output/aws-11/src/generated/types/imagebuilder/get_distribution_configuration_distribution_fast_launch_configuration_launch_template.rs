#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
    /// ID of the Amazon EC2 launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<String>,
    /// The name of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Box<String>,
    /// The version of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateVersion")]
    pub r#launch_template_version: Box<String>,
}
