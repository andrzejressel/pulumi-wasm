#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}
