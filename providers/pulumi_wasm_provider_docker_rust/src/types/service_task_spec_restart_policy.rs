#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecRestartPolicy {
    /// Condition for restart
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Delay between restart attempts (ms|s|m|h)
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Maximum attempts to restart a given container before giving up (default value is `0`, which is ignored)
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    /// The time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}
