#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    #[serde(rename = "genericResources")]
    pub r#generic_resources:
        Box<Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>>,
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}
