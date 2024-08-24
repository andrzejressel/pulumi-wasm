#[derive(serde::Serialize)]
pub struct ContainerPort {
    /// Port exposed out of the container. If not given a free random port `>= 32768` will be used.
    #[serde(rename = "external")]
    pub r#external: Box<Option<i32>>,
    /// Port within the container.
    #[serde(rename = "internal")]
    pub r#internal: Box<i32>,
    /// IP address/mask that can access this port. Defaults to `0.0.0.0`.
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// Protocol that can be used over this port. Defaults to `tcp`.
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
