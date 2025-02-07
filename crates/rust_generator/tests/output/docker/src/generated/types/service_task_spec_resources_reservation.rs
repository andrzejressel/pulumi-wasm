#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecResourcesReservation {
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, GPU=UUID1)
    #[builder(into, default)]
    #[serde(rename = "genericResources")]
    pub r#generic_resources: Box<Option<super::types::ServiceTaskSpecResourcesReservationGenericResources>>,
    /// The amounf of memory in bytes the container allocates
    #[builder(into, default)]
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    /// CPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`
    #[builder(into, default)]
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}
