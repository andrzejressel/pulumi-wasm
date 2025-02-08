#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareNodePoolConfig {
    /// VMware disk size to be used during creation.
    #[builder(into, default)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Box<Option<i32>>,
    /// The number of CPUs for each node in the node pool.
    #[builder(into, default)]
    #[serde(rename = "cpus")]
    pub r#cpus: Box<Option<i32>>,
    /// Allow node pool traffic to be load balanced. Only works for clusters with
    /// MetalLB load balancers.
    #[builder(into, default)]
    #[serde(rename = "enableLoadBalancer")]
    pub r#enable_load_balancer: Box<Option<bool>>,
    /// The OS image name in vCenter, only valid when using Windows.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// The OS image to be used for each node in a node pool.
    /// Currently `cos`, `cos_cgv2`, `ubuntu`, `ubuntu_cgv2`, `ubuntu_containerd` and `windows` are supported.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<String>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node.
    /// These will added in addition to any default label(s) that
    /// Kubernetes may apply to the node.
    /// In case of conflict in label keys, the applied set may differ depending on
    /// the Kubernetes version -- it's best to assume the behavior is undefined
    /// and conflicts should be avoided.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The megabytes of memory for each node in the node pool.
    #[builder(into, default)]
    #[serde(rename = "memoryMb")]
    pub r#memory_mb: Box<Option<i32>>,
    /// The number of nodes in the node pool.
    #[builder(into, default)]
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
    /// The initial taints assigned to nodes of this node pool.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "taints")]
    pub r#taints: Box<Option<Vec<super::super::types::gkeonprem::VMwareNodePoolConfigTaint>>>,
    /// Specifies the vSphere config for node pool.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vsphereConfig")]
    pub r#vsphere_config: Box<Option<super::super::types::gkeonprem::VMwareNodePoolConfigVsphereConfig>>,
}
