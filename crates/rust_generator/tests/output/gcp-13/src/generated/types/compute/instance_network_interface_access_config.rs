#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceNetworkInterfaceAccessConfig {
    /// The IP address that will be 1:1 mapped to the instance's
    /// network ip. If not given, one will be generated.
    #[builder(into, default)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Box<Option<String>>,
    /// The service-level to be provided for IPv6 traffic when the
    /// subnet has an external subnet. Only PREMIUM or STANDARD tier is valid for IPv6.
    #[builder(into, default)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<Option<String>>,
    /// The domain name to be used when creating DNSv6
    /// records for the external IPv6 ranges..
    #[builder(into, default)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Box<Option<String>>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into, default)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<Option<String>>,
}
