#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceEndpointSpecPort {
    /// A random name for the port
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Rrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Represents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`.
    #[builder(into, default)]
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Box<Option<String>>,
    /// The port on the swarm hosts
    #[builder(into, default)]
    #[serde(rename = "publishedPort")]
    pub r#published_port: Box<Option<i32>>,
    /// The port inside the container
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
}
