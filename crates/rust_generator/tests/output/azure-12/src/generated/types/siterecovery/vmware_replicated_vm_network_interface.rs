#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VmwareReplicatedVmNetworkInterface {
    /// Whether this `network_interface` is primary for the replicated VM.
    #[builder(into)]
    #[serde(rename = "isPrimary")]
    pub r#is_primary: Box<bool>,
    /// Mac address of the network interface of source VM.
    #[builder(into)]
    #[serde(rename = "sourceMacAddress")]
    pub r#source_mac_address: Box<String>,
    /// Static IP to assign when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Box<Option<String>>,
    /// Name of the subnet to use when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Box<Option<String>>,
    /// Name of the subnet to use when a test failover is done.
    #[builder(into, default)]
    #[serde(rename = "testSubnetName")]
    pub r#test_subnet_name: Box<Option<String>>,
}
