#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerScheduleScheduleMonthly {
    /// The occurrence of the specified day during the month. For example, a `monthly` property with `weekday` and `week` values of `Sunday, -1` means the last Sunday of the month.
    #[builder(into, default)]
    #[serde(rename = "week")]
    pub r#week: Box<Option<i32>>,
    /// The day of the week on which the trigger runs. For example, a `monthly` property with a `weekday` value of `Sunday` means every Sunday of the month.
    #[builder(into)]
    #[serde(rename = "weekday")]
    pub r#weekday: Box<String>,
}
