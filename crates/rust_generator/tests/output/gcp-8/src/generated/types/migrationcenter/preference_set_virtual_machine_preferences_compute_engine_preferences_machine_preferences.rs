#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferences {
    /// Compute Engine machine series to consider for insights and recommendations. If empty, no restriction is applied on the machine series.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowedMachineSeries")]
    pub r#allowed_machine_series: Box<Option<Vec<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferencesAllowedMachineSeries>>>,
}
