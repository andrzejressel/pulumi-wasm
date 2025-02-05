#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceFromMachineImageNetworkInterfaceAccessConfig {
    /// The IP address that is be 1:1 mapped to the instance's network ip.
    #[builder(into, default)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Box<Option<String>>,
    /// The networking tier used for configuring this instance. One of PREMIUM or STANDARD.
    #[builder(into, default)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<Option<String>>,
    /// The DNS domain name for the public PTR record.
    #[builder(into, default)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Box<Option<String>>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into, default)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<Option<String>>,
}
