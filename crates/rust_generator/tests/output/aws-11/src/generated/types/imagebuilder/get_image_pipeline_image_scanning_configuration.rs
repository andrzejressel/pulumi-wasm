#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImagePipelineImageScanningConfiguration {
    /// List if an object with ecr configuration for image scanning
    #[builder(into)]
    #[serde(rename = "ecrConfigurations")]
    pub r#ecr_configurations: Box<Vec<super::super::types::imagebuilder::GetImagePipelineImageScanningConfigurationEcrConfiguration>>,
    /// Whether image scanning is enabled.
    #[builder(into)]
    #[serde(rename = "imageScanningEnabled")]
    pub r#image_scanning_enabled: Box<bool>,
}
