#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcEndpointDnsEntry {
    /// DNS name.
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Box<String>,
    /// ID of the private hosted zone.
    #[builder(into)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Box<String>,
}
