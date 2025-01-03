#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceDnsConfigDnsRecord {
    /// Amount of time, in seconds, that you want DNS resolvers to cache the settings for this resource record set.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
    /// The type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
