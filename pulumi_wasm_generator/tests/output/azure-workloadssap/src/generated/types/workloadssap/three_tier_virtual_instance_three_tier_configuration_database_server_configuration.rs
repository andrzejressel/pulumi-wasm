#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfiguration {
    /// The database type for the Database Server. Possible values are `DB2` and `HANA`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "databaseType")]
    pub r#database_type: Box<Option<String>>,
    /// One or more `disk_volume_configuration` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diskVolumeConfigurations")]
    pub r#disk_volume_configurations: Box<Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfigurationDiskVolumeConfiguration>>>,
    /// The number of instances for the Database Server. Possible values are at least `1`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<i32>,
    /// The resource ID of the Subnet for the Database Server. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A `virtual_machine_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfiguration")]
    pub r#virtual_machine_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfigurationVirtualMachineConfiguration>,
}
