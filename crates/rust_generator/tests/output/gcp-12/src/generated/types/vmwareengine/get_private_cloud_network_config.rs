#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudNetworkConfig {
    /// DNS Server IP of the Private Cloud.
    #[builder(into)]
    #[serde(rename = "dnsServerIp")]
    pub r#dns_server_ip: Box<String>,
    /// Management CIDR used by VMware management appliances.
    #[builder(into)]
    #[serde(rename = "managementCidr")]
    pub r#management_cidr: Box<String>,
    /// The IP address layout version of the management IP address range.
    /// Possible versions include:
    /// * managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds
    /// as it does not support all features.
    /// * managementIpAddressLayoutVersion=2: Indicates the latest IP address layout
    /// used by all newly created private clouds. This version supports all current features.
    #[builder(into)]
    #[serde(rename = "managementIpAddressLayoutVersion")]
    pub r#management_ip_address_layout_version: Box<i32>,
    /// The relative resource name of the VMware Engine network attached to the private cloud.
    /// Specify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
    /// where {project} can either be a project number or a project ID.
    #[builder(into)]
    #[serde(rename = "vmwareEngineNetwork")]
    pub r#vmware_engine_network: Box<String>,
    /// The canonical name of the VMware Engine network in
    /// the form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
    #[builder(into)]
    #[serde(rename = "vmwareEngineNetworkCanonical")]
    pub r#vmware_engine_network_canonical: Box<String>,
}
