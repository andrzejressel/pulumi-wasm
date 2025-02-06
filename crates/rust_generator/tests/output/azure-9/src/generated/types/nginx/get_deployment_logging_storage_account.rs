#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDeploymentLoggingStorageAccount {
    /// The container name of Storage Account for logging.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// The name of this NGINX Deployment.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
