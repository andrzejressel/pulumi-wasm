#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageImageScanningConfiguration {
    /// Configuration block with ECR configuration.
    #[builder(into)]
    #[serde(rename = "ecrConfigurations")]
    pub r#ecr_configurations: Box<Vec<super::super::types::imagebuilder::GetImageImageScanningConfigurationEcrConfiguration>>,
    /// Indicates whether Image Builder keeps a snapshot of the vulnerability scans that Amazon Inspector runs against the build instance when you create a new image.
    #[builder(into)]
    #[serde(rename = "imageScanningEnabled")]
    pub r#image_scanning_enabled: Box<bool>,
}