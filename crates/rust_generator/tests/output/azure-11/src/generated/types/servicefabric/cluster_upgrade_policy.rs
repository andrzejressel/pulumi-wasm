#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterUpgradePolicy {
    /// A `delta_health_policy` block as defined below
    #[builder(into, default)]
    #[serde(rename = "deltaHealthPolicy")]
    pub r#delta_health_policy: Box<Option<super::super::types::servicefabric::ClusterUpgradePolicyDeltaHealthPolicy>>,
    /// Indicates whether to restart the Service Fabric node even if only dynamic configurations have changed.
    #[builder(into, default)]
    #[serde(rename = "forceRestartEnabled")]
    pub r#force_restart_enabled: Box<Option<bool>>,
    /// Specifies the duration, in "hh:mm:ss" string format, after which Service Fabric retries the health check if the previous health check fails. Defaults to `00:45:00`.
    #[builder(into, default)]
    #[serde(rename = "healthCheckRetryTimeout")]
    pub r#health_check_retry_timeout: Box<Option<String>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits in order to verify that the cluster is stable before it continues to the next upgrade domain or completes the upgrade. This wait duration prevents undetected changes of health right after the health check is performed. Defaults to `00:01:00`.
    #[builder(into, default)]
    #[serde(rename = "healthCheckStableDuration")]
    pub r#health_check_stable_duration: Box<Option<String>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits before it performs the initial health check after it finishes the upgrade on the upgrade domain. Defaults to `00:00:30`.
    #[builder(into, default)]
    #[serde(rename = "healthCheckWaitDuration")]
    pub r#health_check_wait_duration: Box<Option<String>>,
    /// A `health_policy` block as defined below
    #[builder(into, default)]
    #[serde(rename = "healthPolicy")]
    pub r#health_policy: Box<Option<super::super::types::servicefabric::ClusterUpgradePolicyHealthPolicy>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric takes to upgrade a single upgrade domain. After this period, the upgrade fails. Defaults to `02:00:00`.
    #[builder(into, default)]
    #[serde(rename = "upgradeDomainTimeout")]
    pub r#upgrade_domain_timeout: Box<Option<String>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits for a replica set to reconfigure into a safe state, if it is not already in a safe state, before Service Fabric proceeds with the upgrade. Defaults to `10675199.02:48:05.4775807`.
    #[builder(into, default)]
    #[serde(rename = "upgradeReplicaSetCheckTimeout")]
    pub r#upgrade_replica_set_check_timeout: Box<Option<String>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric takes for the entire upgrade. After this period, the upgrade fails. Defaults to `12:00:00`.
    #[builder(into, default)]
    #[serde(rename = "upgradeTimeout")]
    pub r#upgrade_timeout: Box<Option<String>>,
}
