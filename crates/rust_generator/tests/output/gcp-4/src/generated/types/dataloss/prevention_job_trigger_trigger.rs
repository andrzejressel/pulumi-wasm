#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerTrigger {
    /// For use with hybrid jobs. Jobs must be manually created and finished.
    #[builder(into, default)]
    #[serde(rename = "manual")]
    pub r#manual: Box<Option<super::super::types::dataloss::PreventionJobTriggerTriggerManual>>,
    /// Schedule for triggered jobs
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<super::super::types::dataloss::PreventionJobTriggerTriggerSchedule>>,
}
