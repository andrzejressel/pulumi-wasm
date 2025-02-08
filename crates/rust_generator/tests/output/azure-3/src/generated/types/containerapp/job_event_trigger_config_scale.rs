#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobEventTriggerConfigScale {
    /// Maximum number of job executions that are created for a trigger.
    #[builder(into, default)]
    #[serde(rename = "maxExecutions")]
    pub r#max_executions: Box<Option<i32>>,
    /// Minimum number of job executions that are created for a trigger.
    #[builder(into, default)]
    #[serde(rename = "minExecutions")]
    pub r#min_executions: Box<Option<i32>>,
    /// Interval to check each event source in seconds.
    #[builder(into, default)]
    #[serde(rename = "pollingIntervalInSeconds")]
    pub r#polling_interval_in_seconds: Box<Option<i32>>,
    /// A `rules` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::super::types::containerapp::JobEventTriggerConfigScaleRule>>>,
}
