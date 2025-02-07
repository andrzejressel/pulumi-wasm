#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitHostNetwork {
    /// One or more `intent` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "intents")]
    pub r#intents: Box<Vec<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntent>>,
    /// Whether allows users to specify IPs and Mask for Storage NICs when Network ATC is not assigning the IPs for storage automatically. Optional parameter required only for [3 nodes switchless deployments](https://learn.microsoft.com/azure-stack/hci/concepts/physical-network-requirements?tabs=overview%2C23H2reqs#using-switchless). Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "storageAutoIpEnabled")]
    pub r#storage_auto_ip_enabled: Box<Option<bool>>,
    /// Defines how the storage adapters between nodes are connected either switch or switch less. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "storageConnectivitySwitchlessEnabled")]
    pub r#storage_connectivity_switchless_enabled: Box<Option<bool>>,
    /// One or more `storage_network` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storageNetworks")]
    pub r#storage_networks: Box<Vec<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkStorageNetwork>>,
}
