#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodeConfigLinuxNodeConfig {
    /// Possible cgroup modes that can be used.
    /// Accepted values are:
    /// * `CGROUP_MODE_UNSPECIFIED`: CGROUP_MODE_UNSPECIFIED is when unspecified cgroup configuration is used. The default for the GKE node OS image will be used.
    /// * `CGROUP_MODE_V1`: CGROUP_MODE_V1 specifies to use cgroupv1 for the cgroup configuration on the node image.
    /// * `CGROUP_MODE_V2`: CGROUP_MODE_V2 specifies to use cgroupv2 for the cgroup configuration on the node image.
    #[builder(into, default)]
    #[serde(rename = "cgroupMode")]
    pub r#cgroup_mode: Box<Option<String>>,
    /// Amounts for 2M and 1G hugepages. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hugepagesConfig")]
    pub r#hugepages_config: Box<Option<super::super::types::container::ClusterNodeConfigLinuxNodeConfigHugepagesConfig>>,
    /// The Linux kernel parameters to be applied to the nodes
    /// and all pods running on the nodes. Specified as a map from the key, such as
    /// `net.core.wmem_max`, to a string value. Currently supported attributes can be found [here](https://cloud.google.com/sdk/gcloud/reference/beta/container/node-pools/create#--system-config-from-file).
    /// Note that validations happen all server side. All attributes are optional.
    /// 
    #[builder(into, default)]
    #[serde(rename = "sysctls")]
    pub r#sysctls: Box<Option<std::collections::HashMap<String, String>>>,
}
