#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTrafficManagerProfileDnsConfig {
    /// The relative domain name, this is combined with the domain name used by Traffic Manager to form the FQDN which is exported as documented below.
    #[builder(into)]
    #[serde(rename = "relativeName")]
    pub r#relative_name: Box<String>,
    /// The TTL value of the Profile used by Local DNS resolvers and clients.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}