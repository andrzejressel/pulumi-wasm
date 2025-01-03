#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfig {
    /// The autoscaling policy config associated with the cluster.
    /// Note that once set, if `autoscaling_config` is the only field set in `cluster_config`, it can
    /// only be removed by setting `policy_uri = ""`, rather than removing the whole block.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigAutoscalingConfig>>,
    /// A Dataproc NodeGroup resource is a group of Dataproc cluster nodes that execute an assigned role. 
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "auxiliaryNodeGroups")]
    pub r#auxiliary_node_groups: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroup>>>,
    /// The name of the cloud storage bucket ultimately used to house the staging data
    /// for the cluster. If `staging_bucket` is specified, it will contain this value, otherwise
    /// it will be the auto generated name.
    #[builder(into, default)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<Option<String>>,
    /// The Compute Engine accelerator (GPU) configuration for these instances. Can be specified multiple times.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "dataprocMetricConfig")]
    pub r#dataproc_metric_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigDataprocMetricConfig>>,
    /// The Customer managed encryption keys settings for the cluster.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigEncryptionConfig>>,
    /// The config settings for port access on the cluster.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "endpointConfig")]
    pub r#endpoint_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigEndpointConfig>>,
    /// Common config settings for resources of Google Compute Engine cluster
    /// instances, applicable to all instances in the cluster. Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "gceClusterConfig")]
    pub r#gce_cluster_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigGceClusterConfig>>,
    /// Commands to execute on each node after config is completed.
    /// You can specify multiple versions of these. Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "initializationActions")]
    pub r#initialization_actions: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigInitializationAction>>>,
    /// The settings for auto deletion cluster schedule.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfig")]
    pub r#lifecycle_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigLifecycleConfig>>,
    /// The Google Compute Engine config settings for the master instances
    /// in a cluster. Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "masterConfig")]
    pub r#master_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigMasterConfig>>,
    /// The config setting for metastore service with the cluster.
    /// Structure defined below.
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "metastoreConfig")]
    pub r#metastore_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigMetastoreConfig>>,
    /// The Google Compute Engine config settings for the additional
    /// instances in a cluster. Structure defined below.
    /// * **NOTE** : `preemptible_worker_config` is
    /// an alias for the api's [secondaryWorkerConfig](https://cloud.google.com/dataproc/docs/reference/rest/v1/ClusterConfig#InstanceGroupConfig). The name doesn't necessarily mean it is preemptible and is named as
    /// such for legacy/compatibility reasons.
    #[builder(into, default)]
    #[serde(rename = "preemptibleWorkerConfig")]
    pub r#preemptible_worker_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfig>>,
    /// Security related configuration. Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigSecurityConfig>>,
    /// The config settings for software inside the cluster.
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "softwareConfig")]
    pub r#software_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigSoftwareConfig>>,
    /// The Cloud Storage staging bucket used to stage files,
    /// such as Hadoop jars, between client machines and the cluster.
    /// Note: If you don't explicitly specify a `staging_bucket`
    /// then GCP will auto create / assign one for you. However, you are not guaranteed
    /// an auto generated bucket which is solely dedicated to your cluster; it may be shared
    /// with other clusters in the same region/zone also choosing to use the auto generation
    /// option.
    #[builder(into, default)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Box<Option<String>>,
    /// The Cloud Storage temp bucket used to store ephemeral cluster
    /// and jobs data, such as Spark and MapReduce history files.
    /// Note: If you don't explicitly specify a `temp_bucket` then GCP will auto create / assign one for you.
    #[builder(into, default)]
    #[serde(rename = "tempBucket")]
    pub r#temp_bucket: Box<Option<String>>,
    /// The Google Compute Engine config settings for the worker instances
    /// in a cluster. Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "workerConfig")]
    pub r#worker_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigWorkerConfig>>,
}
