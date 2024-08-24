#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    /// The amounf of memory in bytes the container allocates
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    /// CPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}
