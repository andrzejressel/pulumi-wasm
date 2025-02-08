#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionConfigurationDistributionLaunchTemplateConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// Indicates whether to set the specified Amazon EC2 launch template as the default launch template. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<bool>>,
    /// The ID of the Amazon EC2 launch template to use.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Box<String>,
}
