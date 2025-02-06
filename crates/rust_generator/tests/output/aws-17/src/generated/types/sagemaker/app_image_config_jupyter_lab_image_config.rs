#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppImageConfigJupyterLabImageConfig {
    /// The configuration used to run the application image container. See Container Config details below.
    #[builder(into, default)]
    #[serde(rename = "containerConfig")]
    pub r#container_config: Box<Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigContainerConfig>>,
    /// The URL where the Git repository is located. See File System Config details below.
    #[builder(into, default)]
    #[serde(rename = "fileSystemConfig")]
    pub r#file_system_config: Box<Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfigFileSystemConfig>>,
}
