#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateNetworkInterface {
    /// Associate a Carrier IP address with `eth0` for a new network interface. Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. Boolean value, can be left unset.
    #[builder(into, default)]
    #[serde(rename = "associateCarrierIpAddress")]
    pub r#associate_carrier_ip_address: Box<Option<String>>,
    /// Associate a public ip address with the network interface. Boolean value, can be left unset.
    #[builder(into, default)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Box<Option<String>>,
    /// Whether the network interface should be destroyed on instance termination.
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<String>>,
    /// Description of the network interface.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The integer index of the network interface attachment.
    #[builder(into, default)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: Box<Option<i32>>,
    /// The type of network interface. To create an Elastic Fabric Adapter (EFA), specify `efa`.
    #[builder(into, default)]
    #[serde(rename = "interfaceType")]
    pub r#interface_type: Box<Option<String>>,
    /// The number of secondary private IPv4 addresses to assign to a network interface. Conflicts with `ipv4_addresses`
    #[builder(into, default)]
    #[serde(rename = "ipv4AddressCount")]
    pub r#ipv_4_address_count: Box<Option<i32>>,
    /// One or more private IPv4 addresses to associate. Conflicts with `ipv4_address_count`
    #[builder(into, default)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Box<Option<Vec<String>>>,
    /// The number of IPv4 prefixes to be automatically assigned to the network interface. Conflicts with `ipv4_prefixes`
    #[builder(into, default)]
    #[serde(rename = "ipv4PrefixCount")]
    pub r#ipv_4_prefix_count: Box<Option<i32>>,
    /// One or more IPv4 prefixes to be assigned to the network interface. Conflicts with `ipv4_prefix_count`
    #[builder(into, default)]
    #[serde(rename = "ipv4Prefixes")]
    pub r#ipv_4_prefixes: Box<Option<Vec<String>>>,
    /// The number of IPv6 addresses to assign to a network interface. Conflicts with `ipv6_addresses`
    #[builder(into, default)]
    #[serde(rename = "ipv6AddressCount")]
    pub r#ipv_6_address_count: Box<Option<i32>>,
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Conflicts with `ipv6_address_count`
    #[builder(into, default)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Box<Option<Vec<String>>>,
    /// The number of IPv6 prefixes to be automatically assigned to the network interface. Conflicts with `ipv6_prefixes`
    #[builder(into, default)]
    #[serde(rename = "ipv6PrefixCount")]
    pub r#ipv_6_prefix_count: Box<Option<i32>>,
    /// One or more IPv6 prefixes to be assigned to the network interface. Conflicts with `ipv6_prefix_count`
    #[builder(into, default)]
    #[serde(rename = "ipv6Prefixes")]
    pub r#ipv_6_prefixes: Box<Option<Vec<String>>>,
    /// The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.
    #[builder(into, default)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: Box<Option<i32>>,
    /// The ID of the network interface to attach.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<Option<String>>,
    /// Whether the first IPv6 GUA will be made the primary IPv6 address.
    #[builder(into, default)]
    #[serde(rename = "primaryIpv6")]
    pub r#primary_ipv_6: Box<Option<String>>,
    /// The primary private IPv4 address.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// A list of security group IDs to associate.
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    /// The VPC Subnet ID to associate.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
