#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LaunchTemplatePrivateDnsNameOptions {
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records.
    #[builder(into, default)]
    #[serde(rename = "enableResourceNameDnsARecord")]
    pub r#enable_resource_name_dns_a_record: Box<Option<bool>>,
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.
    #[builder(into, default)]
    #[serde(rename = "enableResourceNameDnsAaaaRecord")]
    pub r#enable_resource_name_dns_aaaa_record: Box<Option<bool>>,
    /// The type of hostname for Amazon EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 native subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID. Valid values: `ip-name` and `resource-name`.
    #[builder(into, default)]
    #[serde(rename = "hostnameType")]
    pub r#hostname_type: Box<Option<String>>,
}
