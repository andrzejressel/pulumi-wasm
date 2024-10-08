#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    /// Bind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
