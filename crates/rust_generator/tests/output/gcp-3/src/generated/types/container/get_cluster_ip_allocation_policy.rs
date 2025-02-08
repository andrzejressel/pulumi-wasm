#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterIpAllocationPolicy {
    /// AdditionalPodRangesConfig is the configuration for additional pod secondary ranges supporting the ClusterUpdate message.
    #[builder(into)]
    #[serde(rename = "additionalPodRangesConfigs")]
    pub r#additional_pod_ranges_configs: Box<Vec<super::super::types::container::GetClusterIpAllocationPolicyAdditionalPodRangesConfig>>,
    /// The IP address range for the cluster pod IPs. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "clusterIpv4CidrBlock")]
    pub r#cluster_ipv_4_cidr_block: Box<String>,
    /// The name of the existing secondary range in the cluster's subnetwork to use for pod IP addresses. Alternatively, cluster_ipv4_cidr_block can be used to automatically create a GKE-managed one.
    #[builder(into)]
    #[serde(rename = "clusterSecondaryRangeName")]
    pub r#cluster_secondary_range_name: Box<String>,
    /// Configuration for cluster level pod cidr overprovision. Default is disabled=false.
    #[builder(into)]
    #[serde(rename = "podCidrOverprovisionConfigs")]
    pub r#pod_cidr_overprovision_configs: Box<Vec<super::super::types::container::GetClusterIpAllocationPolicyPodCidrOverprovisionConfig>>,
    /// The IP address range of the services IPs in this cluster. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "servicesIpv4CidrBlock")]
    pub r#services_ipv_4_cidr_block: Box<String>,
    /// The name of the existing secondary range in the cluster's subnetwork to use for service ClusterIPs. Alternatively, services_ipv4_cidr_block can be used to automatically create a GKE-managed one.
    #[builder(into)]
    #[serde(rename = "servicesSecondaryRangeName")]
    pub r#services_secondary_range_name: Box<String>,
    /// The IP Stack type of the cluster. Choose between IPV4 and IPV4_IPV6. Default type is IPV4 Only if not set
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Box<String>,
}
