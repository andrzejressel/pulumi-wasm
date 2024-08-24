#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    /// Bind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
