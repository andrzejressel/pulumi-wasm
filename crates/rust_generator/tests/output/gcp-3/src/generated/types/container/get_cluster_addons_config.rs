#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterAddonsConfig {
    /// The status of the CloudRun addon. It is disabled by default. Set disabled = false to enable.
    #[builder(into)]
    #[serde(rename = "cloudrunConfigs")]
    pub r#cloudrun_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigCloudrunConfig>>,
    /// The of the Config Connector addon.
    #[builder(into)]
    #[serde(rename = "configConnectorConfigs")]
    pub r#config_connector_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigConfigConnectorConfig>>,
    /// The status of the NodeLocal DNSCache addon. It is disabled by default. Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "dnsCacheConfigs")]
    pub r#dns_cache_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigDnsCacheConfig>>,
    /// Whether this cluster should enable the Google Compute Engine Persistent Disk Container Storage Interface (CSI) Driver. Set enabled = true to enable. The Compute Engine persistent disk CSI Driver is enabled by default on newly created clusters for the following versions: Linux clusters: GKE version 1.18.10-gke.2100 or later, or 1.19.3-gke.2100 or later.
    #[builder(into)]
    #[serde(rename = "gcePersistentDiskCsiDriverConfigs")]
    pub r#gce_persistent_disk_csi_driver_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigGcePersistentDiskCsiDriverConfig>>,
    /// The status of the Filestore CSI driver addon, which allows the usage of filestore instance as volumes. Defaults to disabled for Standard clusters; set enabled = true to enable. It is enabled by default for Autopilot clusters; set enabled = true to enable it explicitly.
    #[builder(into)]
    #[serde(rename = "gcpFilestoreCsiDriverConfigs")]
    pub r#gcp_filestore_csi_driver_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigGcpFilestoreCsiDriverConfig>>,
    /// The status of the GCS Fuse CSI driver addon, which allows the usage of gcs bucket as volumes. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "gcsFuseCsiDriverConfigs")]
    pub r#gcs_fuse_csi_driver_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigGcsFuseCsiDriverConfig>>,
    /// The status of the Backup for GKE Agent addon. It is disabled by default. Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "gkeBackupAgentConfigs")]
    pub r#gke_backup_agent_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigGkeBackupAgentConfig>>,
    /// The status of the Horizontal Pod Autoscaling addon, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods. It ensures that a Heapster pod is running in the cluster, which is also used by the Cloud Monitoring service. It is enabled by default; set disabled = true to disable.
    #[builder(into)]
    #[serde(rename = "horizontalPodAutoscalings")]
    pub r#horizontal_pod_autoscalings: Box<Vec<super::super::types::container::GetClusterAddonsConfigHorizontalPodAutoscaling>>,
    /// The status of the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster. It is enabled by default; set disabled = true to disable.
    #[builder(into)]
    #[serde(rename = "httpLoadBalancings")]
    pub r#http_load_balancings: Box<Vec<super::super::types::container::GetClusterAddonsConfigHttpLoadBalancing>>,
    /// The status of the Istio addon.
    #[builder(into)]
    #[serde(rename = "istioConfigs")]
    pub r#istio_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigIstioConfig>>,
    /// Configuration for the KALM addon, which manages the lifecycle of k8s. It is disabled by default; Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "kalmConfigs")]
    pub r#kalm_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigKalmConfig>>,
    /// Whether we should enable the network policy addon for the master. This must be enabled in order to enable network policy for the nodes. To enable this, you must also define a network_policy block, otherwise nothing will happen. It can only be disabled if the nodes already do not have network policies enabled. Defaults to disabled; set disabled = false to enable.
    #[builder(into)]
    #[serde(rename = "networkPolicyConfigs")]
    pub r#network_policy_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigNetworkPolicyConfig>>,
    /// The status of the Parallelstore CSI driver addon, which allows the usage of Parallelstore instances as volumes. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "parallelstoreCsiDriverConfigs")]
    pub r#parallelstore_csi_driver_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigParallelstoreCsiDriverConfig>>,
    /// The status of the Ray Operator addon, which enabled management of Ray AI/ML jobs on GKE. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayOperatorConfigs")]
    pub r#ray_operator_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigRayOperatorConfig>>,
    /// The status of the Stateful HA addon, which provides automatic configurable failover for stateful applications. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "statefulHaConfigs")]
    pub r#stateful_ha_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigStatefulHaConfig>>,
}
