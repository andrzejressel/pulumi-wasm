#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterDnsConfig {
    /// This will enable Cloud DNS additive VPC scope. Must provide a domain name that is unique within the VPC. For this to work `cluster_dns = "CLOUD_DNS"` and `cluster_dns_scope = "CLUSTER_SCOPE"` must both be set as well.
    #[builder(into, default)]
    #[serde(rename = "additiveVpcScopeDnsDomain")]
    pub r#additive_vpc_scope_dns_domain: Box<Option<String>>,
    /// Which in-cluster DNS provider should be used. `PROVIDER_UNSPECIFIED` (default) or `PLATFORM_DEFAULT` or `CLOUD_DNS`.
    #[builder(into, default)]
    #[serde(rename = "clusterDns")]
    pub r#cluster_dns: Box<Option<String>>,
    /// The suffix used for all cluster service records.
    #[builder(into, default)]
    #[serde(rename = "clusterDnsDomain")]
    pub r#cluster_dns_domain: Box<Option<String>>,
    /// The scope of access to cluster DNS records. `DNS_SCOPE_UNSPECIFIED` (default) or `CLUSTER_SCOPE` or `VPC_SCOPE`.
    #[builder(into, default)]
    #[serde(rename = "clusterDnsScope")]
    pub r#cluster_dns_scope: Box<Option<String>>,
}
