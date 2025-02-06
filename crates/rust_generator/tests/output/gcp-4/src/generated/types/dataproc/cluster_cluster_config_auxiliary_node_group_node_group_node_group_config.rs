#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig {
    /// The Compute Engine accelerator (GPU) configuration for these instances. Can be specified 
    /// multiple times.
    #[builder(into, default)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfigAccelerator>>>,
    /// Disk Config
    #[builder(into, default)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfigDiskConfig>>,
    /// List of auxiliary node group instance names which have been assigned to the cluster.
    #[builder(into, default)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Box<Option<Vec<String>>>,
    /// The name of a Google Compute Engine machine type
    /// to create for the node group. If not specified, GCP will default to a predetermined
    /// computed value (currently `n1-standard-4`).
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The name of a minimum generation of CPU family
    /// for the node group. If not specified, GCP will default to a predetermined computed value
    /// for each zone. See [the guide](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// for details about which CPU families are available (and defaulted) for each zone.
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// Specifies the number of master nodes to create.
    /// Please set a number greater than 0. Node Group must have at least 1 instance.
    #[builder(into, default)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Box<Option<i32>>,
}
