#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterConfigPreemptibleWorkerConfig {
    /// Disk Config
    #[builder(into, default)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigDiskConfig>>,
    /// Instance flexibility Policy allowing a mixture of VM shapes and provisioning models.
    #[builder(into, default)]
    #[serde(rename = "instanceFlexibilityPolicy")]
    pub r#instance_flexibility_policy: Box<Option<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy>>,
    /// List of preemptible instance names which have been assigned
    /// to the cluster.
    #[builder(into, default)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Box<Option<Vec<String>>>,
    /// Specifies the number of preemptible nodes to create.
    /// Defaults to 0.
    #[builder(into, default)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Box<Option<i32>>,
    /// Specifies the preemptibility of the secondary workers. The default value is `PREEMPTIBLE`
    /// Accepted values are:
    /// * PREEMPTIBILITY_UNSPECIFIED
    /// * NON_PREEMPTIBLE
    /// * PREEMPTIBLE
    #[builder(into, default)]
    #[serde(rename = "preemptibility")]
    pub r#preemptibility: Box<Option<String>>,
}
