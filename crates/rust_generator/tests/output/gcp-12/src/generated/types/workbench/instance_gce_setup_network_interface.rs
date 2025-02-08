#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceGceSetupNetworkInterface {
    /// Optional. An array of configurations for this interface. Currently, only one access
    /// config, ONE_TO_ONE_NAT, is supported. If no accessConfigs specified, the
    /// instance will have an external internet access through an ephemeral
    /// external IP address.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Box<Option<Vec<super::super::types::workbench::InstanceGceSetupNetworkInterfaceAccessConfig>>>,
    /// Optional. The name of the VPC that this VM instance is in.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// Optional. The type of vNIC to be used on this interface. This
    /// may be gVNIC or VirtioNet.
    /// Possible values are: `VIRTIO_NET`, `GVNIC`.
    #[builder(into, default)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Box<Option<String>>,
    /// Optional. The name of the subnet that this VM instance is in.
    #[builder(into, default)]
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}
