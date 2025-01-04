#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterNetworkProfile {
    /// IP address within the Kubernetes service address range that will be used by cluster service discovery (kube-dns). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "dnsServiceIp")]
    pub r#dns_service_ip: Box<Option<String>>,
    /// Specifies a list of IP versions the Kubernetes Cluster will use to assign IP addresses to its nodes and pods. Possible values are `IPv4` and/or `IPv6`. `IPv4` must always be specified. Changing this forces a new resource to be created.
    /// 
    /// ->**Note:** To configure dual-stack networking `ip_versions` should be set to `["IPv4", "IPv6"]`.
    /// 
    /// ->**Note:** Dual-stack networking requires that the Preview Feature `Microsoft.ContainerService/AKS-EnableDualStack` is enabled and the Resource Provider is re-registered, see [the documentation](https://docs.microsoft.com/azure/aks/configure-kubenet-dual-stack?tabs=azure-cli%2Ckubectl#register-the-aks-enabledualstack-preview-feature) for more information.
    #[builder(into, default)]
    #[serde(rename = "ipVersions")]
    pub r#ip_versions: Box<Option<Vec<String>>>,
    /// A `load_balancer_profile` block as defined below. This can only be specified when `load_balancer_sku` is set to `standard`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerProfile")]
    pub r#load_balancer_profile: Box<Option<super::super::types::containerservice::KubernetesClusterNetworkProfileLoadBalancerProfile>>,
    /// Specifies the SKU of the Load Balancer used for this Kubernetes Cluster. Possible values are `basic` and `standard`. Defaults to `standard`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerSku")]
    pub r#load_balancer_sku: Box<Option<String>>,
    /// A `nat_gateway_profile` block as defined below. This can only be specified when `load_balancer_sku` is set to `standard` and `outbound_type` is set to `managedNATGateway` or `userAssignedNATGateway`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "natGatewayProfile")]
    pub r#nat_gateway_profile: Box<Option<super::super::types::containerservice::KubernetesClusterNetworkProfileNatGatewayProfile>>,
    /// Specifies the data plane used for building the Kubernetes network. Possible values are `azure` and `cilium`. Defaults to `azure`. Disabling this forces a new resource to be created.
    /// 
    /// > **Note:** When `network_data_plane` is set to `cilium`, the `network_plugin` field can only be set to `azure`.
    /// 
    /// > **Note:** When `network_data_plane` is set to `cilium`, one of either `network_plugin_mode = "overlay"` or `pod_subnet_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "networkDataPlane")]
    pub r#network_data_plane: Box<Option<String>>,
    /// Network mode to be used with Azure CNI. Possible values are `bridge` and `transparent`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** `network_mode` can only be set to `bridge` for existing Kubernetes Clusters and cannot be used to provision new Clusters - this will be removed by Azure in the future.
    /// 
    /// > **Note:** This property can only be set when `network_plugin` is set to `azure`.
    #[builder(into, default)]
    #[serde(rename = "networkMode")]
    pub r#network_mode: Box<Option<String>>,
    /// Network plugin to use for networking. Currently supported values are `azure`, `kubenet` and `none`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** When `network_plugin` is set to `azure` - the `pod_cidr` field must not be set, unless specifying `network_plugin_mode` to `overlay`.
    #[builder(into)]
    #[serde(rename = "networkPlugin")]
    pub r#network_plugin: Box<String>,
    /// Specifies the network plugin mode used for building the Kubernetes network. Possible value is `overlay`.
    /// 
    /// > **Note:** When `network_plugin_mode` is set to `overlay`, the `network_plugin` field can only be set to `azure`. When upgrading from Azure CNI without overlay, `pod_subnet_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "networkPluginMode")]
    pub r#network_plugin_mode: Box<Option<String>>,
    /// Sets up network policy to be used with Azure CNI. [Network policy allows us to control the traffic flow between pods](https://docs.microsoft.com/azure/aks/use-network-policies). Currently supported values are `calico`, `azure` and `cilium`.
    /// 
    /// > **Note:** When `network_policy` is set to `azure`, the `network_plugin` field can only be set to `azure`.
    /// 
    /// > **Note:** When `network_policy` is set to `cilium`, the `network_data_plane` field must be set to `cilium`.
    #[builder(into, default)]
    #[serde(rename = "networkPolicy")]
    pub r#network_policy: Box<Option<String>>,
    /// The outbound (egress) routing method which should be used for this Kubernetes Cluster. Possible values are `loadBalancer`, `userDefinedRouting`, `managedNATGateway` and `userAssignedNATGateway`. Defaults to `loadBalancer`. More information on supported migration paths for `outbound_type` can be found in [this documentation](https://learn.microsoft.com/azure/aks/egress-outboundtype#updating-outboundtype-after-cluster-creation).
    #[builder(into, default)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Box<Option<String>>,
    /// The CIDR to use for pod IP addresses. This field can only be set when `network_plugin` is set to `kubenet` or `network_plugin_mode` is set to `overlay`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: Box<Option<String>>,
    /// A list of CIDRs to use for pod IP addresses. For single-stack networking a single IPv4 CIDR is expected. For dual-stack networking an IPv4 and IPv6 CIDR are expected. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "podCidrs")]
    pub r#pod_cidrs: Box<Option<Vec<String>>>,
    /// The Network Range used by the Kubernetes service. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: Box<Option<String>>,
    /// A list of CIDRs to use for Kubernetes services. For single-stack networking a single IPv4 CIDR is expected. For dual-stack networking an IPv4 and IPv6 CIDR are expected. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This range should not be used by any network element on or connected to this VNet. Service address CIDR must be smaller than /12. `docker_bridge_cidr`, `dns_service_ip` and `service_cidr` should all be empty or all should be set.
    #[builder(into, default)]
    #[serde(rename = "serviceCidrs")]
    pub r#service_cidrs: Box<Option<Vec<String>>>,
}
