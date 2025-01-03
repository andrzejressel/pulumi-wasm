#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerInstanceFlexibilityPolicyInstanceSelection {
    /// Full machine-type names, e.g. "n1-standard-16"
    #[builder(into)]
    #[serde(rename = "machineTypes")]
    pub r#machine_types: Box<Vec<String>>,
    /// The name of the instance group manager. Must be 1-63
    /// characters long and comply with
    /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
    /// include lowercase letters, numbers, and hyphens.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Preference of this instance selection. Lower number means higher preference. MIG will first try to create a VM based on the machine-type with lowest rank and fallback to next rank based on availability. Machine types and instance selections with the same rank have the same preference.
    #[builder(into, default)]
    #[serde(rename = "rank")]
    pub r#rank: Box<Option<i32>>,
}
