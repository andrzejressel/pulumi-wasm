#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigPrivateEnvironmentConfig {
    /// When specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into, default)]
    #[serde(rename = "cloudComposerConnectionSubnetwork")]
    pub r#cloud_composer_connection_subnetwork: Box<Option<String>>,
    /// The CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into, default)]
    #[serde(rename = "cloudComposerNetworkIpv4CidrBlock")]
    pub r#cloud_composer_network_ipv_4_cidr_block: Box<Option<String>>,
    /// The CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block.
    #[builder(into, default)]
    #[serde(rename = "cloudSqlIpv4CidrBlock")]
    pub r#cloud_sql_ipv_4_cidr_block: Box<Option<String>>,
    /// Mode of internal communication within the Composer environment. Must be one of "VPC_PEERING" or "PRIVATE_SERVICE_CONNECT".
    #[builder(into, default)]
    #[serde(rename = "connectionType")]
    pub r#connection_type: Box<Option<String>>,
    /// If true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: Box<Option<bool>>,
    /// When enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block.
    #[builder(into, default)]
    #[serde(rename = "enablePrivatelyUsedPublicIps")]
    pub r#enable_privately_used_public_ips: Box<Option<bool>>,
    /// The IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used.
    #[builder(into, default)]
    #[serde(rename = "masterIpv4CidrBlock")]
    pub r#master_ipv_4_cidr_block: Box<Option<String>>,
    /// The CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "webServerIpv4CidrBlock")]
    pub r#web_server_ipv_4_cidr_block: Box<Option<String>>,
}
