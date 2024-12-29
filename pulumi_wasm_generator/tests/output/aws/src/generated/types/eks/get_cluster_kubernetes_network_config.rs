#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterKubernetesNetworkConfig {
    /// `ipv4` or `ipv6`.
    #[builder(into)]
    #[serde(rename = "ipFamily")]
    pub r#ip_family: Box<String>,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from if `ipv4` was specified when the cluster was created.
    #[builder(into)]
    #[serde(rename = "serviceIpv4Cidr")]
    pub r#service_ipv_4_cidr: Box<String>,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from if `ipv6` was specified when the cluster was created. Kubernetes assigns service addresses from the unique local address range (fc00::/7) because you can't specify a custom IPv6 CIDR block when you create the cluster.
    #[builder(into)]
    #[serde(rename = "serviceIpv6Cidr")]
    pub r#service_ipv_6_cidr: Box<String>,
}
