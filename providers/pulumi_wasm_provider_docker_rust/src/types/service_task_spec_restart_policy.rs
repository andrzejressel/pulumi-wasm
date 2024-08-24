#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    /// Condition for restart
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Delay between restart attempts (ms|s|m|h)
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Maximum attempts to restart a given container before giving up (default value is `0`, which is ignored)
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    /// The time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}
