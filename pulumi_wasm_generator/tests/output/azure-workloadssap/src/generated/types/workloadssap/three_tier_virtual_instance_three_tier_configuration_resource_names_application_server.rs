#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServer {
    /// The full name for the availability set. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "availabilitySetName")]
    pub r#availability_set_name: Box<Option<String>>,
    /// One or more `virtual_machine` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualMachines")]
    pub r#virtual_machines: Box<Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachine>>>,
}
