#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolDefaultsNodeConfigDefaults {
    /// Parameters for containerd configuration.
    #[builder(into, default)]
    #[serde(rename = "containerdConfig")]
    pub r#containerd_config: Box<Option<super::super::types::container::ClusterNodePoolDefaultsNodeConfigDefaultsContainerdConfig>>,
    /// The default Google Container Filesystem (GCFS) configuration at the cluster level. e.g. enable [image streaming](https://cloud.google.com/kubernetes-engine/docs/how-to/image-streaming) across all the node pools within the cluster. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcfsConfig")]
    pub r#gcfs_config: Box<Option<super::super::types::container::ClusterNodePoolDefaultsNodeConfigDefaultsGcfsConfig>>,
    /// Controls whether the kubelet read-only port is enabled for newly created node pools in the cluster. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into, default)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: Box<Option<String>>,
    /// The type of logging agent that is deployed by default for newly created node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT. See [Increasing logging agent throughput](https://cloud.google.com/stackdriver/docs/solutions/gke/managing-logs#throughput) for more information.
    #[builder(into, default)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: Box<Option<String>>,
}
