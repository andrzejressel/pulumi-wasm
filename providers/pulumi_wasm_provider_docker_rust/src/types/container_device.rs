#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ContainerDevice {
    /// The path in the container where the device will be bound.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The path on the host where the device is located.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    /// The cgroup permissions given to the container to access the device. Defaults to `rwm`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}
