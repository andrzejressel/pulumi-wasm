#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerDevice {
    /// The path in the container where the device will be bound.
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The path on the host where the device is located.
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    /// The cgroup permissions given to the container to access the device. Defaults to `rwm`.
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}
