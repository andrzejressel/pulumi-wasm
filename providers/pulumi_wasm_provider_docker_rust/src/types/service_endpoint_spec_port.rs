#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceEndpointSpecPort {
    /// A random name for the port
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Rrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`.
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Represents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`.
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Box<Option<String>>,
    /// The port on the swarm hosts
    #[serde(rename = "publishedPort")]
    pub r#published_port: Box<Option<i32>>,
    /// The port inside the container
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
}
