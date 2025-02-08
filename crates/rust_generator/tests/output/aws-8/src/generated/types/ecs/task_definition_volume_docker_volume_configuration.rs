#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TaskDefinitionVolumeDockerVolumeConfiguration {
    /// If this value is `true`, the Docker volume is created if it does not already exist. *Note*: This field is only used if the scope is `shared`.
    #[builder(into, default)]
    #[serde(rename = "autoprovision")]
    pub r#autoprovision: Box<Option<bool>>,
    /// Docker volume driver to use. The driver value must match the driver name provided by Docker because it is used for task placement.
    #[builder(into, default)]
    #[serde(rename = "driver")]
    pub r#driver: Box<Option<String>>,
    /// Map of Docker driver specific options.
    #[builder(into, default)]
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<std::collections::HashMap<String, String>>>,
    /// Map of custom metadata to add to your Docker volume.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Scope for the Docker volume, which determines its lifecycle, either `task` or `shared`.  Docker volumes that are scoped to a `task` are automatically provisioned when the task starts and destroyed when the task stops. Docker volumes that are scoped as `shared` persist after the task stops.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
}
