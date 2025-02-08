#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceDnsConfig {
    /// An array that contains one DnsRecord object for each resource record set. See `dns_records` Block for details.
    #[builder(into)]
    #[serde(rename = "dnsRecords")]
    pub r#dns_records: Box<Vec<super::super::types::servicediscovery::ServiceDnsConfigDnsRecord>>,
    /// The ID of the namespace to use for DNS configuration.
    #[builder(into)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
    /// The routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify the service. Valid Values: MULTIVALUE, WEIGHTED
    #[builder(into, default)]
    #[serde(rename = "routingPolicy")]
    pub r#routing_policy: Box<Option<String>>,
}
