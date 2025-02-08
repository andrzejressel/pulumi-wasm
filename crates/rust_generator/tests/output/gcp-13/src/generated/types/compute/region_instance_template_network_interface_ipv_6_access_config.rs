#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionInstanceTemplateNetworkInterfaceIpv6AccessConfig {
    /// The first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. The field is output only, an IPv6 address from a subnetwork associated with the instance will be allocated dynamically.
    #[builder(into, default)]
    #[serde(rename = "externalIpv6")]
    pub r#external_ipv_6: Box<Option<String>>,
    /// The prefix length of the external IPv6 range.
    #[builder(into, default)]
    #[serde(rename = "externalIpv6PrefixLength")]
    pub r#external_ipv_6_prefix_length: Box<Option<String>>,
    /// The name of this access configuration.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<String>,
    /// The domain name to be used when creating DNSv6 records for the external IPv6 ranges.
    #[builder(into, default)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Box<Option<String>>,
}
