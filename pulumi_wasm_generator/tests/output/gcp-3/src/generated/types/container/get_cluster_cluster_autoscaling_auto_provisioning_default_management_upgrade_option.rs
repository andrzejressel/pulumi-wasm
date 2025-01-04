#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultManagementUpgradeOption {
    /// This field is set when upgrades are about to commence with the approximate start time for the upgrades, in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "autoUpgradeStartTime")]
    pub r#auto_upgrade_start_time: Box<String>,
    /// This field is set when upgrades are about to commence with the description of the upgrade.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
}
