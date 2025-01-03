#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageImageScanningConfigurationEcrConfiguration {
    /// Set of tags for Image Builder to apply to the output container image that that Amazon Inspector scans.
    #[builder(into, default)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Option<Vec<String>>>,
    /// The name of the container repository that Amazon Inspector scans to identify findings for your container images.
    #[builder(into, default)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<Option<String>>,
}
