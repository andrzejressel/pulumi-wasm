#[derive(serde::Serialize)]
pub struct ServiceEndpointSpecPort {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Box<Option<String>>,
    #[serde(rename = "publishedPort")]
    pub r#published_port: Box<Option<i32>>,
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
}
