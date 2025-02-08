#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    /// Bind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`
    #[builder(into, default)]
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
