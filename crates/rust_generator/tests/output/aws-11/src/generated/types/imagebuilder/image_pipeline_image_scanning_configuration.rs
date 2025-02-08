#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ImagePipelineImageScanningConfiguration {
    /// Configuration block with ECR configuration for image scanning. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ecrConfiguration")]
    pub r#ecr_configuration: Box<Option<super::super::types::imagebuilder::ImagePipelineImageScanningConfigurationEcrConfiguration>>,
    /// Whether image scans are enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "imageScanningEnabled")]
    pub r#image_scanning_enabled: Box<Option<bool>>,
}
