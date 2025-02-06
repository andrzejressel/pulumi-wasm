#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerServiceDeploymentVersionPublicEndpoint {
    /// The name of the container for the endpoint.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// The port of the container to which traffic is forwarded to.
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<i32>,
    /// A configuration block that describes the health check configuration of the container. Detailed below.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<super::super::types::lightsail::ContainerServiceDeploymentVersionPublicEndpointHealthCheck>,
}
