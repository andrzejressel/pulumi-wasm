#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterTmpf {
    /// The absolute file path in the container where the tmpfs volume is mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<String>,
    /// The list of tmpfs volume mount options.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Vec<String>>,
    /// The size (in MiB) of the tmpfs volume.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
}
