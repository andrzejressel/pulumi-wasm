#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AzureNodePoolConfig {
    /// The OS image type to use on node pool instances.
    #[builder(into, default)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<Option<String>>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into, default)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Box<Option<super::super::types::container::AzureNodePoolConfigProxyConfig>>,
    /// Optional. Configuration related to the root volume provisioned for each node pool machine. When unspecified, it defaults to a 32-GiB Azure Disk.
    #[builder(into, default)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Box<Option<super::super::types::container::AzureNodePoolConfigRootVolume>>,
    /// SSH configuration for how to access the node pool machines.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<super::super::types::container::AzureNodePoolConfigSshConfig>,
    /// Optional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`.
    #[builder(into, default)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<Option<String>>,
}
