#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionEksPropertyPodPropertyVolumeEmptyDir {
    /// The medium to store the volume.
    #[builder(into)]
    #[serde(rename = "medium")]
    pub r#medium: Box<String>,
    /// The maximum size of the volume. By default, there's no maximum size defined.
    #[builder(into)]
    #[serde(rename = "sizeLimit")]
    pub r#size_limit: Box<String>,
}