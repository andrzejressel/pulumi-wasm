#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterIpAllocationPolicy {
    /// The configuration for additional pod secondary ranges at
    /// the cluster level. Used for Autopilot clusters and Standard clusters with which control of the
    /// secondary Pod IP address assignment to node pools isn't needed. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalPodRangesConfig")]
    pub r#additional_pod_ranges_config: Box<Option<super::super::types::container::ClusterIpAllocationPolicyAdditionalPodRangesConfig>>,
    /// The IP address range for the cluster pod IPs.
    /// Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14)
    /// to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14)
    /// from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to
    /// pick a specific range to use.
    #[builder(into, default)]
    #[serde(rename = "clusterIpv4CidrBlock")]
    pub r#cluster_ipv_4_cidr_block: Box<Option<String>>,
    /// The name of the existing secondary
    /// range in the cluster's subnetwork to use for pod IP addresses. Alternatively,
    /// `cluster_ipv4_cidr_block` can be used to automatically create a GKE-managed one.
    #[builder(into, default)]
    #[serde(rename = "clusterSecondaryRangeName")]
    pub r#cluster_secondary_range_name: Box<Option<String>>,
    /// Configuration for cluster level pod cidr overprovision. Default is disabled=false.
    #[builder(into, default)]
    #[serde(rename = "podCidrOverprovisionConfig")]
    pub r#pod_cidr_overprovision_config: Box<Option<super::super::types::container::ClusterIpAllocationPolicyPodCidrOverprovisionConfig>>,
    /// The IP address range of the services IPs in this cluster.
    /// Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14)
    /// to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14)
    /// from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to
    /// pick a specific range to use.
    #[builder(into, default)]
    #[serde(rename = "servicesIpv4CidrBlock")]
    pub r#services_ipv_4_cidr_block: Box<Option<String>>,
    /// The name of the existing
    /// secondary range in the cluster's subnetwork to use for service `ClusterIP`s.
    /// Alternatively, `services_ipv4_cidr_block` can be used to automatically create a
    /// GKE-managed one.
    #[builder(into, default)]
    #[serde(rename = "servicesSecondaryRangeName")]
    pub r#services_secondary_range_name: Box<Option<String>>,
    /// The IP Stack Type of the cluster.
    /// Default value is `IPV4`.
    /// Possible values are `IPV4` and `IPV4_IPV6`.
    #[builder(into, default)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Box<Option<String>>,
}
