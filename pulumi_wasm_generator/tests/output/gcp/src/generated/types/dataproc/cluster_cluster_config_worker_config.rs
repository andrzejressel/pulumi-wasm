#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigWorkerConfig {
    /// The Compute Engine accelerator configuration for these instances. Can be specified multiple times.
    #[builder(into, default)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigWorkerConfigAccelerator>>>,
    /// Disk Config
    #[builder(into, default)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigWorkerConfigDiskConfig>>,
    /// The URI for the image to use for this worker.  See [the guide](https://cloud.google.com/dataproc/docs/guides/dataproc-images)
    /// for more information.
    #[builder(into, default)]
    #[serde(rename = "imageUri")]
    pub r#image_uri: Box<Option<String>>,
    /// List of worker instance names which have been assigned
    /// to the cluster.
    #[builder(into, default)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Box<Option<Vec<String>>>,
    /// The name of a Google Compute Engine machine type
    /// to create for the worker nodes. If not specified, GCP will default to a predetermined
    /// computed value (currently `n1-standard-4`).
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The name of a minimum generation of CPU family
    /// for the master. If not specified, GCP will default to a predetermined computed value
    /// for each zone. See [the guide](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// for details about which CPU families are available (and defaulted) for each zone.
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// The minimum number of primary worker instances to create.  If `min_num_instances` is set, cluster creation will succeed if the number of primary workers created is at least equal to the `min_num_instances` number.
    #[builder(into, default)]
    #[serde(rename = "minNumInstances")]
    pub r#min_num_instances: Box<Option<i32>>,
    /// Specifies the number of worker nodes to create.
    /// If not specified, GCP will default to a predetermined computed value (currently 2).
    /// There is currently a beta feature which allows you to run a
    /// [Single Node Cluster](https://cloud.google.com/dataproc/docs/concepts/single-node-clusters).
    /// In order to take advantage of this you need to set
    /// `"dataproc:dataproc.allow.zero.workers" = "true"` in
    /// `cluster_config.software_config.properties`
    #[builder(into, default)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Box<Option<i32>>,
}
