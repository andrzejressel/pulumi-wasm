#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeHost {
    /// The path on the host container instance that's presented to the container.
    #[builder(into)]
    #[serde(rename = "sourcePath")]
    pub r#source_path: Box<String>,
}
