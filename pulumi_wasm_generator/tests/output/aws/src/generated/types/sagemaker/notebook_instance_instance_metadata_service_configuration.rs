#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NotebookInstanceInstanceMetadataServiceConfiguration {
    /// Indicates the minimum IMDS version that the notebook instance supports. When passed "1" is passed. This means that both IMDSv1 and IMDSv2 are supported. Valid values are `1` and `2`.
    #[builder(into, default)]
    #[serde(rename = "minimumInstanceMetadataServiceVersion")]
    pub r#minimum_instance_metadata_service_version: Box<Option<String>>,
}