#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsParameters {
    /// Indicates whether to exclude the root volume from snapshots created using CreateSnapshots. The default is `false`.
    #[builder(into, default)]
    #[serde(rename = "excludeBootVolume")]
    pub r#exclude_boot_volume: Box<Option<bool>>,
    /// Applies to AMI lifecycle policies only. Indicates whether targeted instances are rebooted when the lifecycle policy runs. `true` indicates that targeted instances are not rebooted when the policy runs. `false` indicates that target instances are rebooted when the policy runs. The default is `true` (instances are not rebooted).
    #[builder(into, default)]
    #[serde(rename = "noReboot")]
    pub r#no_reboot: Box<Option<bool>>,
}
