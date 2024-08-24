#[derive(serde::Serialize)]
pub struct ContainerHost {
    /// Hostname to add
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// IP address this hostname should resolve to.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
