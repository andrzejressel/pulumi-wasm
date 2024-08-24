#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    /// The name of the host
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The ip of the host
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
