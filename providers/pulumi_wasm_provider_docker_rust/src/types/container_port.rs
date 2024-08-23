#[derive(serde::Serialize)]
pub struct ContainerPort {
    #[serde(rename = "external")]
    pub r#external: Box<Option<i32>>,
    #[serde(rename = "internal")]
    pub r#internal: Box<i32>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
