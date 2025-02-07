#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainAutoTuneOptionsMaintenanceSchedule {
    /// A cron expression specifying the recurrence pattern for an Auto-Tune maintenance schedule.
    #[builder(into)]
    #[serde(rename = "cronExpressionForRecurrence")]
    pub r#cron_expression_for_recurrence: Box<String>,
    /// Configuration block for the duration of the Auto-Tune maintenance window. Detailed below.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<super::super::types::opensearch::DomainAutoTuneOptionsMaintenanceScheduleDuration>,
    /// Date and time at which to start the Auto-Tune maintenance schedule in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "startAt")]
    pub r#start_at: Box<String>,
}
