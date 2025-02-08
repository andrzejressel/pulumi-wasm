#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigMasterConfig {
    /// The Compute Engine accelerator configuration for these instances.
    #[builder(into, default)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigAccelerator>>>,
    /// Disk option config settings.
    #[builder(into, default)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigDiskConfig>>,
    /// The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// Output only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group.
    #[builder(into, default)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Box<Option<Vec<String>>>,
    /// Output only. Specifies that this instance group contains preemptible instances.
    #[builder(into, default)]
    #[serde(rename = "isPreemptible")]
    pub r#is_preemptible: Box<Option<bool>>,
    /// The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/(https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Output only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups.
    #[builder(into, default)]
    #[serde(rename = "managedGroupConfigs")]
    pub r#managed_group_configs: Box<Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigManagedGroupConfig>>>,
    /// Specifies the minimum cpu platform for the Instance Group. See [Minimum CPU platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu).
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// The number of VM instances in the instance group. For master instance groups, must be set to 1.
    #[builder(into, default)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Box<Option<i32>>,
    /// Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE
    #[builder(into, default)]
    #[serde(rename = "preemptibility")]
    pub r#preemptibility: Box<Option<String>>,
}
