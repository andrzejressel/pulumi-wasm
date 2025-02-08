#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistribution {
    /// Nested list of AMI distribution configuration.
    #[builder(into)]
    #[serde(rename = "amiDistributionConfigurations")]
    pub r#ami_distribution_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionAmiDistributionConfiguration>>,
    /// Nested list of container distribution configurations.
    #[builder(into)]
    #[serde(rename = "containerDistributionConfigurations")]
    pub r#container_distribution_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionContainerDistributionConfiguration>>,
    /// Nested list of Windows faster-launching configurations to use for AMI distribution.
    #[builder(into)]
    #[serde(rename = "fastLaunchConfigurations")]
    pub r#fast_launch_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfiguration>>,
    /// Nested list of launch template configurations.
    #[builder(into)]
    #[serde(rename = "launchTemplateConfigurations")]
    pub r#launch_template_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionLaunchTemplateConfiguration>>,
    /// Set of Amazon Resource Names (ARNs) of License Manager License Configurations.
    #[builder(into)]
    #[serde(rename = "licenseConfigurationArns")]
    pub r#license_configuration_arns: Box<Vec<String>>,
    /// AWS Region of distribution.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// Nested list of S3 export configuration.
    #[builder(into)]
    #[serde(rename = "s3ExportConfigurations")]
    pub r#s_3_export_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionS3ExportConfiguration>>,
}
