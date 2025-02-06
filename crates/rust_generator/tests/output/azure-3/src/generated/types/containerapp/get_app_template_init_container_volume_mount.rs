#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppTemplateInitContainerVolumeMount {
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The path in the container at which to mount this volume.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
