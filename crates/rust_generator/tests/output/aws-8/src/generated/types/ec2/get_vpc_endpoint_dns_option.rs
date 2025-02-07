#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcEndpointDnsOption {
    /// The DNS records created for the endpoint.
    #[builder(into)]
    #[serde(rename = "dnsRecordIpType")]
    pub r#dns_record_ip_type: Box<String>,
    /// Indicates whether to enable private DNS only for inbound endpoints.
    #[builder(into)]
    #[serde(rename = "privateDnsOnlyForInboundResolverEndpoint")]
    pub r#private_dns_only_for_inbound_resolver_endpoint: Box<bool>,
}
