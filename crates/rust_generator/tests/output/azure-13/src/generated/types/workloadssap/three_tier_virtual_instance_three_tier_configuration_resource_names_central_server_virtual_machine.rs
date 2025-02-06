#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerVirtualMachine {
    /// One or more `data_disk` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "dataDisks")]
    pub r#data_disks: Box<Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerVirtualMachineDataDisk>>>,
    /// The full name of the host of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<Option<String>>,
    /// A list of full names for the Network Interface of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceNames")]
    pub r#network_interface_names: Box<Option<Vec<String>>>,
    /// The full name of the OS Disk attached to the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "osDiskName")]
    pub r#os_disk_name: Box<Option<String>>,
    /// The full name of the Virtual Machine in a single server SAP system. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualMachineName")]
    pub r#virtual_machine_name: Box<Option<String>>,
}
