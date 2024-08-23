#[derive(serde::Serialize)]
pub struct ContainerHost {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
