#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceNetworkInterfaceIpv6AccessConfig {
    /// The first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. To use a static external IP address, it must be unused and in the same region as the instance's zone. If not specified, Google Cloud will automatically assign an external IPv6 address from the instance's subnetwork.
    #[builder(into)]
    #[serde(rename = "externalIpv6")]
    pub r#external_ipv_6: Box<String>,
    /// The prefix length of the external IPv6 range.
    #[builder(into)]
    #[serde(rename = "externalIpv6PrefixLength")]
    pub r#external_ipv_6_prefix_length: Box<String>,
    /// The name of the instance. One of `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The [networking tier][network-tier] used for configuring this instance. One of `PREMIUM` or `STANDARD`.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<String>,
    /// The DNS domain name for the public PTR record.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Box<String>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<String>,
}
