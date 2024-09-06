#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The container where the volume is coming from.
    #[serde(rename = "fromContainer")]
    pub r#from_container: Box<Option<String>>,
    /// The path on the host where the volume is coming from.
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<String>>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// The name of the docker volume which should be mounted.
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<Option<String>>,
}
