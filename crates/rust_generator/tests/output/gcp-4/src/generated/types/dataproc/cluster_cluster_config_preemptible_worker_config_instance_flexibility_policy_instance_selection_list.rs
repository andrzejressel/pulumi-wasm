#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionList {
    /// Full machine-type names, e.g. `"n1-standard-16"`.
    #[builder(into, default)]
    #[serde(rename = "machineTypes")]
    pub r#machine_types: Box<Option<Vec<String>>>,
    /// Preference of this instance selection. A lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "rank")]
    pub r#rank: Box<Option<i32>>,
}
