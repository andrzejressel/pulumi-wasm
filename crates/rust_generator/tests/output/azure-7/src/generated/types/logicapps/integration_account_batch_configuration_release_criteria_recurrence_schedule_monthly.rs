#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceScheduleMonthly {
    /// The occurrence of the week within the month.
    #[builder(into)]
    #[serde(rename = "week")]
    pub r#week: Box<i32>,
    /// The day of the occurrence. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekday")]
    pub r#weekday: Box<String>,
}
