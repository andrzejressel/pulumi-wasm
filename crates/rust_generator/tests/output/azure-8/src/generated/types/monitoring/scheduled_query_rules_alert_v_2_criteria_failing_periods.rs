#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledQueryRulesAlertV2CriteriaFailingPeriods {
    /// Specifies the number of violations to trigger an alert. Should be smaller or equal to `number_of_evaluation_periods`. Possible value is integer between 1 and 6.
    #[builder(into)]
    #[serde(rename = "minimumFailingPeriodsToTriggerAlert")]
    pub r#minimum_failing_periods_to_trigger_alert: Box<i32>,
    /// Specifies the number of aggregated look-back points. The look-back time window is calculated based on the aggregation granularity `window_duration` and the selected number of aggregated points. Possible value is integer between 1 and 6.
    /// 
    /// > **Note** The query look back which is `window_duration`*`number_of_evaluation_periods` cannot exceed 48 hours.
    /// 
    /// > **Note** `number_of_evaluation_periods` must be `1` for queries that do not project timestamp column
    #[builder(into)]
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub r#number_of_evaluation_periods: Box<i32>,
}
