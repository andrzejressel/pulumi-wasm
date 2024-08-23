#[derive(serde::Serialize)]
pub struct ContainerDevice {
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}
