#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterConfigGceClusterConfig {
    /// Confidential Instance Config for clusters using [Confidential VMs](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/confidential-compute)
    #[builder(into, default)]
    #[serde(rename = "confidentialInstanceConfig")]
    pub r#confidential_instance_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigConfidentialInstanceConfig>>,
    /// By default, clusters are not restricted to internal IP addresses,
    /// and will have ephemeral external IP addresses assigned to each instance. If set to true, all
    /// instances in the cluster will only have internal IP addresses. Note: Private Google Access
    /// (also known as `privateIpGoogleAccess`) must be enabled on the subnetwork that the cluster
    /// will be launched in.
    #[builder(into, default)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Box<Option<bool>>,
    /// A map of the Compute Engine metadata entries to add to all instances
    /// (see [Project and instance metadata](https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata)).
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name or self_link of the Google Compute Engine
    /// network to the cluster will be part of. Conflicts with `subnetwork`.
    /// If neither is specified, this defaults to the "default" network.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// Node Group Affinity for sole-tenant clusters.
    #[builder(into, default)]
    #[serde(rename = "nodeGroupAffinity")]
    pub r#node_group_affinity: Box<Option<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigNodeGroupAffinity>>,
    /// Reservation Affinity for consuming zonal reservation.
    #[builder(into, default)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Box<Option<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigReservationAffinity>>,
    /// The service account to be used by the Node VMs.
    /// If not specified, the "default" service account is used.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// The set of Google API scopes
    /// to be made available on all of the node VMs under the `service_account`
    /// specified. Both OAuth2 URLs and gcloud
    /// short names are supported. To allow full access to all Cloud APIs, use the
    /// `cloud-platform` scope. See a complete list of scopes [here](https://cloud.google.com/sdk/gcloud/reference/alpha/compute/instances/set-scopes#--scopes).
    #[builder(into, default)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Box<Option<Vec<String>>>,
    /// Shielded Instance Config for clusters using [Compute Engine Shielded VMs](https://cloud.google.com/security/shielded-cloud/shielded-vm).
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigShieldedInstanceConfig>>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork the cluster will be part of. Conflicts with `network`.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
    /// The list of instance tags applied to instances in the cluster.
    /// Tags are used to identify valid sources or targets for network firewalls.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// The GCP zone where your data is stored and used (i.e. where
    /// the master and the worker nodes will be created in). If `region` is set to 'global' (default)
    /// then `zone` is mandatory, otherwise GCP is able to make use of [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/auto-zone)
    /// to determine this automatically for you.
    /// Note: This setting additionally determines and restricts
    /// which computing resources are available for use with other configs such as
    /// `cluster_config.master_config.machine_type` and `cluster_config.worker_config.machine_type`.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
