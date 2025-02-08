#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSubnetworksSubnetwork {
    /// Description of the subnetwork.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The IP address range represented as a CIDR block.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Box<String>,
    /// The name of the subnetwork.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The self link of the parent network.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    #[builder(into)]
    #[serde(rename = "networkSelfLink")]
    pub r#network_self_link: Box<String>,
    /// Whether the VMs in the subnet can access Google services without assigned external IP addresses.
    #[builder(into)]
    #[serde(rename = "privateIpGoogleAccess")]
    pub r#private_ip_google_access: Box<bool>,
    /// The self link of the subnetwork.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
}
