#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageImageScanningConfiguration {
    /// Configuration block with ECR configuration. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ecrConfiguration")]
    pub r#ecr_configuration: Box<Option<super::super::types::imagebuilder::ImageImageScanningConfigurationEcrConfiguration>>,
    /// Indicates whether Image Builder keeps a snapshot of the vulnerability scans that Amazon Inspector runs against the build instance when you create a new image. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "imageScanningEnabled")]
    pub r#image_scanning_enabled: Box<Option<bool>>,
}
