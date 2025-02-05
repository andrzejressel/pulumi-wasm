#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineAdditionalCapabilities {
    /// Whether to enable the hibernation capability or not.
    #[builder(into, default)]
    #[serde(rename = "hibernationEnabled")]
    pub r#hibernation_enabled: Box<Option<bool>>,
    /// Should the capacity to enable Data Disks of the `UltraSSD_LRS` storage account type be supported on this Virtual Machine? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ultraSsdEnabled")]
    pub r#ultra_ssd_enabled: Box<Option<bool>>,
}
