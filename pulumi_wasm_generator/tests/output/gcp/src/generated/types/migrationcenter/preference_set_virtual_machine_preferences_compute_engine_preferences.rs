#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreferenceSetVirtualMachinePreferencesComputeEnginePreferences {
    /// License type to consider when calculating costs for virtual machine insights and recommendations. If unspecified, costs are calculated based on the default licensing plan. Possible values: `LICENSE_TYPE_UNSPECIFIED`, `LICENSE_TYPE_DEFAULT`, `LICENSE_TYPE_BRING_YOUR_OWN_LICENSE`
    #[builder(into, default)]
    #[serde(rename = "licenseType")]
    pub r#license_type: Box<Option<String>>,
    /// The type of machines to consider when calculating virtual machine migration insights and recommendations. Not all machine types are available in all zones and regions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "machinePreferences")]
    pub r#machine_preferences: Box<Option<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferences>>,
}
