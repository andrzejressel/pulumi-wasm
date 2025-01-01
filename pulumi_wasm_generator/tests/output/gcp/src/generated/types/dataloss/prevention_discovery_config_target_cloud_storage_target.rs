#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTarget {
    /// In addition to matching the filter, these conditions must be true before a profile is generated.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetConditions>>,
    /// Disable profiling for buckets that match this filter.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetDisabled>>,
    /// The buckets the generation_cadence applies to. The first target with a matching filter will be the one to apply to a bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilter>,
    /// How often and when to update profiles. New buckets that match both the filter and conditions are scanned as quickly as possible depending on system capacity.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "generationCadence")]
    pub r#generation_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetGenerationCadence>>,
}
