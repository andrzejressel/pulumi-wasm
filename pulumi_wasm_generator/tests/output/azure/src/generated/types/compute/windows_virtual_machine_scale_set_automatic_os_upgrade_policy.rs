#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsVirtualMachineScaleSetAutomaticOsUpgradePolicy {
    /// Should automatic rollbacks be disabled?
    #[builder(into)]
    #[serde(rename = "disableAutomaticRollback")]
    pub r#disable_automatic_rollback: Box<bool>,
    /// Should OS Upgrades automatically be applied to Scale Set instances in a rolling fashion when a newer version of the OS Image becomes available?
    #[builder(into)]
    #[serde(rename = "enableAutomaticOsUpgrade")]
    pub r#enable_automatic_os_upgrade: Box<bool>,
}
