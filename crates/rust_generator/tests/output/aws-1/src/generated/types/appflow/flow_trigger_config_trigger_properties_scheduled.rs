#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowTriggerConfigTriggerPropertiesScheduled {
    /// Whether a scheduled flow has an incremental data transfer or a complete data transfer for each flow run. Valid values are `Incremental` and `Complete`.
    #[builder(into, default)]
    #[serde(rename = "dataPullMode")]
    pub r#data_pull_mode: Box<Option<String>>,
    /// Date range for the records to import from the connector in the first flow run. Must be a valid RFC3339 timestamp.
    #[builder(into, default)]
    #[serde(rename = "firstExecutionFrom")]
    pub r#first_execution_from: Box<Option<String>>,
    /// Scheduled end time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
    #[builder(into, default)]
    #[serde(rename = "scheduleEndTime")]
    pub r#schedule_end_time: Box<Option<String>>,
    /// Scheduling expression that determines the rate at which the schedule will run, for example `rate(5minutes)`.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Box<String>,
    /// Optional offset that is added to the time interval for a schedule-triggered flow. Maximum value of 36000.
    #[builder(into, default)]
    #[serde(rename = "scheduleOffset")]
    pub r#schedule_offset: Box<Option<i32>>,
    /// Scheduled start time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
    #[builder(into, default)]
    #[serde(rename = "scheduleStartTime")]
    pub r#schedule_start_time: Box<Option<String>>,
    /// Time zone used when referring to the date and time of a scheduled-triggered flow, such as `America/New_York`.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: aws:appflow:Flow
    ///     properties:
    ///       triggerConfig:
    ///         scheduled:
    ///           - scheduleExpression: rate(1minutes)
    /// ```
    #[builder(into, default)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<Option<String>>,
}
