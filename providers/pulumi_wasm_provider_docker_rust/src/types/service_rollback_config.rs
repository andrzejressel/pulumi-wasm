#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceRollbackConfig {
    /// Delay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Action on rollback failure: pause | continue. Defaults to `pause`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    /// Failure rate to tolerate during a rollback. Defaults to `0.0`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    /// Duration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    /// Rollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// Maximum number of tasks to be rollbacked in one iteration. Defaults to `1`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}
