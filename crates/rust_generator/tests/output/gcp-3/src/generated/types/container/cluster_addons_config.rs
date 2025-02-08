#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterAddonsConfig {
    /// . Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudrunConfig")]
    pub r#cloudrun_config: Box<Option<super::super::types::container::ClusterAddonsConfigCloudrunConfig>>,
    /// .
    /// The status of the ConfigConnector addon. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into, default)]
    #[serde(rename = "configConnectorConfig")]
    pub r#config_connector_config: Box<Option<super::super::types::container::ClusterAddonsConfigConfigConnectorConfig>>,
    /// .
    /// The status of the NodeLocal DNSCache addon. It is disabled by default.
    /// Set `enabled = true` to enable.
    /// 
    /// **Enabling/Disabling NodeLocal DNSCache in an existing cluster is a disruptive operation.
    /// All cluster nodes running GKE 1.15 and higher are recreated.**
    #[builder(into, default)]
    #[serde(rename = "dnsCacheConfig")]
    pub r#dns_cache_config: Box<Option<super::super::types::container::ClusterAddonsConfigDnsCacheConfig>>,
    /// .
    /// Whether this cluster should enable the Google Compute Engine Persistent Disk Container Storage Interface (CSI) Driver. Set `enabled = true` to enable.
    /// 
    /// **Note:** The Compute Engine persistent disk CSI Driver is enabled by default on newly created clusters for the following versions: Linux clusters: GKE version 1.18.10-gke.2100 or later, or 1.19.3-gke.2100 or later.
    #[builder(into, default)]
    #[serde(rename = "gcePersistentDiskCsiDriverConfig")]
    pub r#gce_persistent_disk_csi_driver_config: Box<Option<super::super::types::container::ClusterAddonsConfigGcePersistentDiskCsiDriverConfig>>,
    /// The status of the Filestore CSI driver addon,
    /// which allows the usage of filestore instance as volumes.
    /// It is disabled by default; set `enabled = true` to enable.
    #[builder(into, default)]
    #[serde(rename = "gcpFilestoreCsiDriverConfig")]
    pub r#gcp_filestore_csi_driver_config: Box<Option<super::super::types::container::ClusterAddonsConfigGcpFilestoreCsiDriverConfig>>,
    /// The status of the GCSFuse CSI driver addon,
    /// which allows the usage of a gcs bucket as volumes.
    /// It is disabled by default for Standard clusters; set `enabled = true` to enable.
    /// It is enabled by default for Autopilot clusters with version 1.24 or later; set `enabled = true` to enable it explicitly.
    /// See [Enable the Cloud Storage FUSE CSI driver](https://cloud.google.com/kubernetes-engine/docs/how-to/persistent-volumes/cloud-storage-fuse-csi-driver#enable) for more information.
    #[builder(into, default)]
    #[serde(rename = "gcsFuseCsiDriverConfig")]
    pub r#gcs_fuse_csi_driver_config: Box<Option<super::super::types::container::ClusterAddonsConfigGcsFuseCsiDriverConfig>>,
    /// .
    /// The status of the Backup for GKE agent addon. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into, default)]
    #[serde(rename = "gkeBackupAgentConfig")]
    pub r#gke_backup_agent_config: Box<Option<super::super::types::container::ClusterAddonsConfigGkeBackupAgentConfig>>,
    /// The status of the Horizontal Pod Autoscaling
    /// addon, which increases or decreases the number of replica pods a replication controller
    /// has based on the resource usage of the existing pods.
    /// It is enabled by default;
    /// set `disabled = true` to disable.
    #[builder(into, default)]
    #[serde(rename = "horizontalPodAutoscaling")]
    pub r#horizontal_pod_autoscaling: Box<Option<super::super::types::container::ClusterAddonsConfigHorizontalPodAutoscaling>>,
    /// The status of the HTTP (L7) load balancing
    /// controller addon, which makes it easy to set up HTTP load balancers for services in a
    /// cluster. It is enabled by default; set `disabled = true` to disable.
    #[builder(into, default)]
    #[serde(rename = "httpLoadBalancing")]
    pub r#http_load_balancing: Box<Option<super::super::types::container::ClusterAddonsConfigHttpLoadBalancing>>,
    /// .
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "istioConfig")]
    pub r#istio_config: Box<Option<super::super::types::container::ClusterAddonsConfigIstioConfig>>,
    /// .
    /// Configuration for the KALM addon, which manages the lifecycle of k8s. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into, default)]
    #[serde(rename = "kalmConfig")]
    pub r#kalm_config: Box<Option<super::super::types::container::ClusterAddonsConfigKalmConfig>>,
    /// Whether we should enable the network policy addon
    /// for the master.  This must be enabled in order to enable network policy for the nodes.
    /// To enable this, you must also define a `network_policy` block,
    /// otherwise nothing will happen.
    /// It can only be disabled if the nodes already do not have network policies enabled.
    /// Defaults to disabled; set `disabled = false` to enable.
    #[builder(into, default)]
    #[serde(rename = "networkPolicyConfig")]
    pub r#network_policy_config: Box<Option<super::super::types::container::ClusterAddonsConfigNetworkPolicyConfig>>,
    /// The status of the Parallelstore CSI driver addon,
    /// which allows the usage of a Parallelstore instances as volumes.
    /// It is disabled by default for Standard clusters; set `enabled = true` to enable.
    /// It is enabled by default for Autopilot clusters with version 1.29 or later; set `enabled = true` to enable it explicitly.
    /// See [Enable the Parallelstore CSI driver](https://cloud.google.com/kubernetes-engine/docs/how-to/persistent-volumes/parallelstore-csi-new-volume#enable) for more information.
    /// 
    /// This example `addons_config` disables two addons:
    /// 
    #[builder(into, default)]
    #[serde(rename = "parallelstoreCsiDriverConfig")]
    pub r#parallelstore_csi_driver_config: Box<Option<super::super::types::container::ClusterAddonsConfigParallelstoreCsiDriverConfig>>,
    /// . The status of the [Ray Operator
    /// addon](https://cloud.google.com/kubernetes-engine/docs/add-on/ray-on-gke/concepts/overview).
    /// It is disabled by default. Set `enabled = true` to enable. The minimum
    /// cluster version to enable Ray is 1.30.0-gke.1747000.
    /// 
    /// Ray Operator config has optional subfields
    /// `ray_cluster_logging_config.enabled` and
    /// `ray_cluster_monitoring_config.enabled` which control Ray Cluster logging
    /// and monitoring respectively. See [Collect and view logs and metrics for Ray
    /// clusters on
    /// GKE](https://cloud.google.com/kubernetes-engine/docs/add-on/ray-on-gke/how-to/collect-view-logs-metrics)
    /// for more information.
    #[builder(into, default)]
    #[serde(rename = "rayOperatorConfigs")]
    pub r#ray_operator_configs: Box<Option<Vec<super::super::types::container::ClusterAddonsConfigRayOperatorConfig>>>,
    /// .
    /// The status of the Stateful HA addon, which provides automatic configurable failover for stateful applications.
    /// It is disabled by default for Standard clusters. Set `enabled = true` to enable.
    #[builder(into, default)]
    #[serde(rename = "statefulHaConfig")]
    pub r#stateful_ha_config: Box<Option<super::super::types::container::ClusterAddonsConfigStatefulHaConfig>>,
}
