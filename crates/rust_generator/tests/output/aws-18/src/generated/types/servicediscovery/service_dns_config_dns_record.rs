#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceDnsConfigDnsRecord {
    /// The amount of time, in seconds, that you want DNS resolvers to cache the settings for this resource record set.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
    /// The type of the resource, which indicates the value that Amazon Route 53 returns in response to DNS queries. Valid Values: A, AAAA, SRV, CNAME
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
