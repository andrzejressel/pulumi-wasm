#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    /// Time between running the check (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    /// Consecutive failures needed to report unhealthy. Defaults to `0`
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    /// Start period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    /// The test to perform as list
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    /// Maximum time to allow one check to run (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
