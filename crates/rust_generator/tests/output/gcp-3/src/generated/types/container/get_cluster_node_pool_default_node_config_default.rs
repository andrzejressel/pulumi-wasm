#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolDefaultNodeConfigDefault {
    /// Parameters for containerd configuration.
    #[builder(into)]
    #[serde(rename = "containerdConfigs")]
    pub r#containerd_configs: Box<Vec<super::super::types::container::GetClusterNodePoolDefaultNodeConfigDefaultContainerdConfig>>,
    /// GCFS configuration for this node.
    #[builder(into)]
    #[serde(rename = "gcfsConfigs")]
    pub r#gcfs_configs: Box<Vec<super::super::types::container::GetClusterNodePoolDefaultNodeConfigDefaultGcfsConfig>>,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: Box<String>,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: Box<String>,
}
