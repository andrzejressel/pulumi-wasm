#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceDnsConfig {
    /// An array that contains one DnsRecord object for each resource record set. See `dns_records` Block for details.
    #[builder(into)]
    #[serde(rename = "dnsRecords")]
    pub r#dns_records: Box<Vec<super::super::types::servicediscovery::GetServiceDnsConfigDnsRecord>>,
    /// ID of the namespace that the service belongs to.
    #[builder(into)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
    /// Routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify the service. Valid Values: MULTIVALUE, WEIGHTED
    #[builder(into)]
    #[serde(rename = "routingPolicy")]
    pub r#routing_policy: Box<String>,
}
