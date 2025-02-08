#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentConfigNodeConfig {
    /// IPv4 cidr range that will be used by Composer internal components.
    #[builder(into, default)]
    #[serde(rename = "composerInternalIpv4CidrBlock")]
    pub r#composer_internal_ipv_4_cidr_block: Box<Option<String>>,
    /// PSC (Private Service Connect) Network entry point. Customers can pre-create the Network Attachment and point Cloud Composer environment to use. It is possible to share network attachment among many environments, provided enough IP addresses are available.
    #[builder(into, default)]
    #[serde(rename = "composerNetworkAttachment")]
    pub r#composer_network_attachment: Box<Option<String>>,
    /// The disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Deploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent
    #[builder(into, default)]
    #[serde(rename = "enableIpMasqAgent")]
    pub r#enable_ip_masq_agent: Box<Option<bool>>,
    /// Configuration for controlling how IPs are allocated in the GKE cluster. Cannot be updated.
    #[builder(into, default)]
    #[serde(rename = "ipAllocationPolicy")]
    pub r#ip_allocation_policy: Box<Option<super::super::types::composer::EnvironmentConfigNodeConfigIpAllocationPolicy>>,
    /// The Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: "projects/{project}/zones/{zone}/machineTypes/{machineType}". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The maximum pods per node in the GKE cluster allocated during environment creation. Lowering this value reduces IP address consumption by the Cloud Composer Kubernetes cluster. This value can only be set during environment creation, and only if the environment is VPC-Native. The range of possible values is 8-110, and the default is 32. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<Option<i32>>,
    /// The Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: "projects/{project}/zones/{zone}/machineTypes/{machineType}". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the "default" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to ["https://www.googleapis.com/auth/cloud-platform"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Option<Vec<String>>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the "default" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// The Compute Engine subnetwork to be used for machine communications, specified as a self-link, relative resource name (e.g. "projects/{project}/regions/{region}/subnetworks/{subnetwork}"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
    /// The list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// The Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. "projects/{project}/zones/{zone}"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
