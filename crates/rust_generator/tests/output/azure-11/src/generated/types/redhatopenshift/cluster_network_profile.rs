#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNetworkProfile {
    /// The outbound (egress) routing method. Possible values are `Loadbalancer` and `UserDefinedRouting`. Defaults to `Loadbalancer`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Box<Option<String>>,
    /// The CIDR to use for pod IP addresses. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: Box<String>,
    /// Whether a preconfigured network security group is being used on the subnets.  Defaults to `false`.  Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "preconfiguredNetworkSecurityGroupEnabled")]
    pub r#preconfigured_network_security_group_enabled: Box<Option<bool>>,
    /// The network range used by the OpenShift service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: Box<String>,
}
