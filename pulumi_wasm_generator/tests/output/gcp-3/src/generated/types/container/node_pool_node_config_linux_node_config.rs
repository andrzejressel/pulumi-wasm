#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolNodeConfigLinuxNodeConfig {
    /// cgroupMode specifies the cgroup mode to be used on the node.
    #[builder(into, default)]
    #[serde(rename = "cgroupMode")]
    pub r#cgroup_mode: Box<Option<String>>,
    /// Amounts for 2M and 1G hugepages.
    #[builder(into, default)]
    #[serde(rename = "hugepagesConfig")]
    pub r#hugepages_config: Box<Option<super::super::types::container::NodePoolNodeConfigLinuxNodeConfigHugepagesConfig>>,
    /// The Linux kernel parameters to be applied to the nodes and all pods running on the nodes.
    #[builder(into, default)]
    #[serde(rename = "sysctls")]
    pub r#sysctls: Box<Option<std::collections::HashMap<String, String>>>,
}
