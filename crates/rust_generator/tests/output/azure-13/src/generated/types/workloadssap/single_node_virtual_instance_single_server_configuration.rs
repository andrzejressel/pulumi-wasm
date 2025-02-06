#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SingleNodeVirtualInstanceSingleServerConfiguration {
    #[builder(into)]
    #[serde(rename = "appResourceGroupName")]
    pub r#app_resource_group_name: Box<String>,
    /// The supported SAP database type. Possible values are `DB2` and `HANA`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "databaseType")]
    pub r#database_type: Box<Option<String>>,
    /// One or more `disk_volume_configuration` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diskVolumeConfigurations")]
    pub r#disk_volume_configurations: Box<Option<Vec<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration>>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "secondaryIpEnabled")]
    pub r#secondary_ip_enabled: Box<Option<bool>>,
    /// The resource ID of the Subnet for the SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A `virtual_machine_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfiguration")]
    pub r#virtual_machine_configuration: Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfiguration>,
    /// A `virtual_machine_resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualMachineResourceNames")]
    pub r#virtual_machine_resource_names: Box<Option<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineResourceNames>>,
}
