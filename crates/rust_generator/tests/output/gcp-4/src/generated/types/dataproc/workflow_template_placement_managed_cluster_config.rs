#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementManagedClusterConfig {
    /// Autoscaling config for the policy associated with the cluster. Cluster does not autoscale if this field is unset.
    #[builder(into, default)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigAutoscalingConfig>>,
    /// Encryption settings for the cluster.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigEncryptionConfig>>,
    /// Port/endpoint configuration for this cluster
    #[builder(into, default)]
    #[serde(rename = "endpointConfig")]
    pub r#endpoint_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigEndpointConfig>>,
    /// The shared Compute Engine config settings for all instances in a cluster.
    #[builder(into, default)]
    #[serde(rename = "gceClusterConfig")]
    pub r#gce_cluster_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig>>,
    /// The Kubernetes Engine config for Dataproc clusters deployed to Kubernetes. Setting this is considered mutually exclusive with Compute Engine-based options such as `gce_cluster_config`, `master_config`, `worker_config`, `secondary_worker_config`, and `autoscaling_config`.
    #[builder(into, default)]
    #[serde(rename = "gkeClusterConfig")]
    pub r#gke_cluster_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGkeClusterConfig>>,
    /// Commands to execute on each node after config is completed. By default, executables are run on master and all worker nodes. You can test a node's `role` metadata to run an executable on a master or worker node, as shown below using `curl` (you can also use `wget`): ROLE=$(curl -H Metadata-Flavor:Google http://metadata/computeMetadata/v1/instance/attributes/dataproc-role) if ; then ... master specific actions ... else ... worker specific actions ... fi
    #[builder(into, default)]
    #[serde(rename = "initializationActions")]
    pub r#initialization_actions: Box<Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigInitializationAction>>>,
    /// Lifecycle setting for the cluster.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfig")]
    pub r#lifecycle_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig>>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    #[builder(into, default)]
    #[serde(rename = "masterConfig")]
    pub r#master_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfig>>,
    /// Metastore configuration.
    #[builder(into, default)]
    #[serde(rename = "metastoreConfig")]
    pub r#metastore_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMetastoreConfig>>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    #[builder(into, default)]
    #[serde(rename = "secondaryWorkerConfig")]
    pub r#secondary_worker_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSecondaryWorkerConfig>>,
    /// Security settings for the cluster.
    #[builder(into, default)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSecurityConfig>>,
    /// The config settings for software inside the cluster.
    #[builder(into, default)]
    #[serde(rename = "softwareConfig")]
    pub r#software_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig>>,
    /// A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see [Dataproc staging and temp buckets](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)).
    #[builder(into, default)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Box<Option<String>>,
    /// A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket.
    #[builder(into, default)]
    #[serde(rename = "tempBucket")]
    pub r#temp_bucket: Box<Option<String>>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "workerConfig")]
    pub r#worker_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigWorkerConfig>>,
}
