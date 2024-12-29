#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobDefinitionEksPropertiesPodPropertiesVolumeHostPath {
    /// Path of the file or directory on the host to mount into containers on the pod.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
