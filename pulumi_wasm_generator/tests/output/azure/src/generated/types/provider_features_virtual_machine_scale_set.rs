#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeaturesVirtualMachineScaleSet {
    #[builder(into, default)]
    #[serde(rename = "forceDelete")]
    pub r#force_delete: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "reimageOnManualUpgrade")]
    pub r#reimage_on_manual_upgrade: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "rollInstancesWhenRequired")]
    pub r#roll_instances_when_required: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "scaleToZeroBeforeDeletion")]
    pub r#scale_to_zero_before_deletion: Box<Option<bool>>,
}
