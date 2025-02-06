#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerServiceDeploymentVersionContainer {
    /// The launch command for the container. A list of string.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// The name for the container.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// A key-value map of the environment variables of the container.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the image used for the container. Container images sourced from your Lightsail container service, that are registered and stored on your service, start with a colon (`:`). For example, `:container-service-1.mystaticwebsite.1`. Container images sourced from a public registry like Docker Hub don't start with a colon. For example, `nginx:latest` or `nginx`.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// A key-value map of the open firewall ports of the container. Valid values: `HTTP`, `HTTPS`, `TCP`, `UDP`.
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<std::collections::HashMap<String, String>>>,
}
