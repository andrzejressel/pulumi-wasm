#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateInitContainerVolumeMount {
    /// The name of the volume to mount. This must match the name of a volume defined in the `volume` block.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The path within the container at which the volume should be mounted. Must not contain `:`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
