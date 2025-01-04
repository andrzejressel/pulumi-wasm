#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolStartTaskContainer {
    /// The image to use to create the container in which the task will run. This is the full image reference, as would be specified to "docker pull". If no tag is provided as part of the image name, the tag ":latest" is used as a default.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<String>,
    /// The `container_registries` block defined as below.
    #[builder(into, default)]
    #[serde(rename = "registries")]
    pub r#registries: Box<Option<Vec<super::super::types::batch::PoolStartTaskContainerRegistry>>>,
    /// Additional options to the container create command. These additional options are supplied as arguments to the "docker create" command, in addition to those controlled by the Batch Service.
    #[builder(into, default)]
    #[serde(rename = "runOptions")]
    pub r#run_options: Box<Option<String>>,
    /// A flag to indicate where the container task working directory is. Possible values are `TaskWorkingDirectory` and `ContainerImageDefault`.
    #[builder(into, default)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: Box<Option<String>>,
}
