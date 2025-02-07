#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicatedVmNetworkInterface {
    /// Id of the public IP object to use when a test failover is done.
    #[builder(into, default)]
    #[serde(rename = "failoverTestPublicIpAddressId")]
    pub r#failover_test_public_ip_address_id: Box<Option<String>>,
    /// Static IP to assign when a test failover is done.
    #[builder(into, default)]
    #[serde(rename = "failoverTestStaticIp")]
    pub r#failover_test_static_ip: Box<Option<String>>,
    /// Name of the subnet to to use when a test failover is done.
    #[builder(into, default)]
    #[serde(rename = "failoverTestSubnetName")]
    pub r#failover_test_subnet_name: Box<Option<String>>,
    /// Id of the public IP object to use when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "recoveryPublicIpAddressId")]
    pub r#recovery_public_ip_address_id: Box<Option<String>>,
    /// (Required if the network_interface block is specified) Id source network interface.
    #[builder(into, default)]
    #[serde(rename = "sourceNetworkInterfaceId")]
    pub r#source_network_interface_id: Box<Option<String>>,
    /// Static IP to assign when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Box<Option<String>>,
    /// Name of the subnet to to use when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Box<Option<String>>,
}
