#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstancePrivateDnsNameOption {
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records.
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsARecord")]
    pub r#enable_resource_name_dns_a_record: Box<bool>,
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsAaaaRecord")]
    pub r#enable_resource_name_dns_aaaa_record: Box<bool>,
    /// Type of hostname for EC2 instances.
    #[builder(into)]
    #[serde(rename = "hostnameType")]
    pub r#hostname_type: Box<String>,
}