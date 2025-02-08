#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcEndpointDnsOptions {
    /// The DNS records created for the endpoint. Valid values are `ipv4`, `dualstack`, `service-defined`, and `ipv6`.
    #[builder(into, default)]
    #[serde(rename = "dnsRecordIpType")]
    pub r#dns_record_ip_type: Box<Option<String>>,
    /// Indicates whether to enable private DNS only for inbound endpoints. This option is available only for services that support both gateway and interface endpoints. It routes traffic that originates from the VPC to the gateway endpoint and traffic that originates from on-premises to the interface endpoint. Default is `false`. Can only be specified if private_dns_enabled is `true`.
    #[builder(into, default)]
    #[serde(rename = "privateDnsOnlyForInboundResolverEndpoint")]
    pub r#private_dns_only_for_inbound_resolver_endpoint: Box<Option<bool>>,
}
