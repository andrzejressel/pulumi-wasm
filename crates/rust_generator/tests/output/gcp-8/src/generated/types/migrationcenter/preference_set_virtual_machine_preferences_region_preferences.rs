#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreferenceSetVirtualMachinePreferencesRegionPreferences {
    /// A list of preferred regions, ordered by the most preferred region first. Set only valid Google Cloud region names. See https://cloud.google.com/compute/docs/regions-zones for available regions.
    #[builder(into, default)]
    #[serde(rename = "preferredRegions")]
    pub r#preferred_regions: Box<Option<Vec<String>>>,
}
