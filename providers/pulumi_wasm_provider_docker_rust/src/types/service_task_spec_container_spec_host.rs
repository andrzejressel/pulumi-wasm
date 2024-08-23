#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
