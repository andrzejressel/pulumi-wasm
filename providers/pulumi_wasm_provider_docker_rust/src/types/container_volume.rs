#[derive(serde::Serialize)]
pub struct ContainerVolume {
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    #[serde(rename = "fromContainer")]
    pub r#from_container: Box<Option<String>>,
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<String>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<Option<String>>,
}
