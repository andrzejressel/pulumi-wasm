#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceGroupManagerInstanceFlexibilityPolicyInstanceSelection {
    /// Full machine-type names, e.g. "n1-standard-16"
    #[builder(into)]
    #[serde(rename = "machineTypes")]
    pub r#machine_types: Box<Vec<String>>,
    /// The name of the instance group. Either `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Preference of this instance selection. Lower number means higher preference. MIG will first try to create a VM based on the machine-type with lowest rank and fallback to next rank based on availability. Machine types and instance selections with the same rank have the same preference.
    #[builder(into)]
    #[serde(rename = "rank")]
    pub r#rank: Box<i32>,
}
