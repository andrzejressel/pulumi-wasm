#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodePoolAutoConfig {
    /// Linux node configuration options.
    #[builder(into)]
    #[serde(rename = "linuxNodeConfigs")]
    pub r#linux_node_configs: Box<Vec<super::super::types::container::GetClusterNodePoolAutoConfigLinuxNodeConfig>>,
    /// Collection of Compute Engine network tags that can be applied to a node's underlying VM instance.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Box<Vec<super::super::types::container::GetClusterNodePoolAutoConfigNetworkTag>>,
    /// Node kubelet configs.
    #[builder(into)]
    #[serde(rename = "nodeKubeletConfigs")]
    pub r#node_kubelet_configs: Box<Vec<super::super::types::container::GetClusterNodePoolAutoConfigNodeKubeletConfig>>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<std::collections::HashMap<String, String>>,
}
