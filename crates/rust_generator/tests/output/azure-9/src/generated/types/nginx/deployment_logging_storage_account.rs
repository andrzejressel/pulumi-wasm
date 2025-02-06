#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentLoggingStorageAccount {
    /// Specify the container name in the Storage Account for logging.
    #[builder(into, default)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<Option<String>>,
    /// The name of the StorageAccount for NGINX Logging.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
