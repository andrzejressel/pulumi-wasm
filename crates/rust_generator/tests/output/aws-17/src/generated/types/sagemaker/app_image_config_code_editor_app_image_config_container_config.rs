#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppImageConfigCodeEditorAppImageConfigContainerConfig {
    /// The arguments for the container when you're running the application.
    #[builder(into, default)]
    #[serde(rename = "containerArguments")]
    pub r#container_arguments: Box<Option<Vec<String>>>,
    /// The entrypoint used to run the application in the container.
    #[builder(into, default)]
    #[serde(rename = "containerEntrypoints")]
    pub r#container_entrypoints: Box<Option<Vec<String>>>,
    /// The environment variables to set in the container.
    #[builder(into, default)]
    #[serde(rename = "containerEnvironmentVariables")]
    pub r#container_environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
}
