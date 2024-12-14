#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The container where the volume is coming from.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "fromContainer")]
    pub r#from_container: Box<Option<String>>,
    /// The path on the host where the volume is coming from.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<String>>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// The name of the docker volume which should be mounted.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<Option<String>>,
}
