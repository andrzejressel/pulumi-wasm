#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainAutoTuneOptionMaintenanceSchedule {
    /// Cron expression for an Auto-Tune maintenance schedule.
    #[builder(into)]
    #[serde(rename = "cronExpressionForRecurrence")]
    pub r#cron_expression_for_recurrence: Box<String>,
    /// Configuration block for the duration of the Auto-Tune maintenance window.
    #[builder(into)]
    #[serde(rename = "durations")]
    pub r#durations: Box<Vec<super::super::types::opensearch::GetDomainAutoTuneOptionMaintenanceScheduleDuration>>,
    /// Date and time at which the Auto-Tune maintenance schedule starts in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "startAt")]
    pub r#start_at: Box<String>,
}