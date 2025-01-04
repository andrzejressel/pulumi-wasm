#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterNetworkProfile {
    /// IP address within the Kubernetes service address range used by cluster service discovery (kube-dns).
    #[builder(into)]
    #[serde(rename = "dnsServiceIp")]
    pub r#dns_service_ip: Box<String>,
    /// IP address (in CIDR notation) used as the Docker bridge IP address on nodes.
    #[builder(into)]
    #[serde(rename = "dockerBridgeCidr")]
    pub r#docker_bridge_cidr: Box<String>,
    #[builder(into)]
    #[serde(rename = "loadBalancerSku")]
    pub r#load_balancer_sku: Box<String>,
    /// Network plugin used such as `azure` or `kubenet`.
    #[builder(into)]
    #[serde(rename = "networkPlugin")]
    pub r#network_plugin: Box<String>,
    /// Network policy to be used with Azure CNI. e.g. `calico` or `azure`
    #[builder(into)]
    #[serde(rename = "networkPolicy")]
    pub r#network_policy: Box<String>,
    /// The CIDR used for pod IP addresses.
    #[builder(into)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: Box<String>,
    /// Network range used by the Kubernetes service.
    #[builder(into)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: Box<String>,
}
