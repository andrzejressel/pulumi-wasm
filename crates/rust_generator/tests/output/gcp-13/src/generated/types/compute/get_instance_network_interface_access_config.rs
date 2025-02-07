#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceNetworkInterfaceAccessConfig {
    /// If the instance has an access config, either the given external ip (in the `nat_ip` field) or the ephemeral (generated) ip (if you didn't provide one).
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Box<String>,
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
