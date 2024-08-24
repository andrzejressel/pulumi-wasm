#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, GPU=UUID1)
    #[serde(rename = "genericResources")]
    pub r#generic_resources: Box<Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>>,
    /// The amounf of memory in bytes the container allocates
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    /// CPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}
