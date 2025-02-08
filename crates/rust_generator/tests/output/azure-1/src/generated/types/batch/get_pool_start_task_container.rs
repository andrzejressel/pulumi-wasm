#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPoolStartTaskContainer {
    /// The image to use to create the container in which the task will run.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<String>,
    /// The same reference as `container_registries` block defined as follows.
    #[builder(into)]
    #[serde(rename = "registries")]
    pub r#registries: Box<Vec<super::super::types::batch::GetPoolStartTaskContainerRegistry>>,
    /// Additional options to the container create command.
    #[builder(into)]
    #[serde(rename = "runOptions")]
    pub r#run_options: Box<String>,
    /// A flag to indicate where the container task working directory is.
    #[builder(into)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: Box<String>,
}
