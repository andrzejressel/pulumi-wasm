#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImagePipelineImageScanningConfigurationEcrConfiguration {
    /// Tags that are added to the output containers that are scanned
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Vec<String>>,
    /// The name of the container repository that Amazon Inspector scans
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
}