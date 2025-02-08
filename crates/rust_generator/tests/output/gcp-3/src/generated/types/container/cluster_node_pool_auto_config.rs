#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolAutoConfig {
    /// Linux system configuration for the cluster's automatically provisioned node pools. Only `cgroup_mode` field is supported in `node_pool_auto_config`. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "linuxNodeConfig")]
    pub r#linux_node_config: Box<Option<super::super::types::container::ClusterNodePoolAutoConfigLinuxNodeConfig>>,
    /// The network tag config for the cluster's automatically provisioned node pools. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Box<Option<super::super::types::container::ClusterNodePoolAutoConfigNetworkTags>>,
    /// Kubelet configuration for Autopilot clusters. Currently, only `insecure_kubelet_readonly_port_enabled` is supported here.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeKubeletConfig")]
    pub r#node_kubelet_config: Box<Option<super::super::types::container::ClusterNodePoolAutoConfigNodeKubeletConfig>>,
    /// A map of resource manager tag keys and values to be attached to the nodes for managing Compute Engine firewalls using Network Firewall Policies. Tags must be according to specifications found [here](https://cloud.google.com/vpc/docs/tags-firewalls-overview#specifications). A maximum of 5 tag key-value pairs can be specified. Existing tags will be replaced with new values. Tags must be in one of the following formats ([KEY]=[VALUE]) 1. `tagKeys/{tag_key_id}=tagValues/{tag_value_id}` 2. `{org_id}/{tag_key_name}={tag_value_name}` 3. `{project_id}/{tag_key_name}={tag_value_name}`.
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
}
