#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule {
    /// A list containing a single item, which specifies the Hour interval at which this recurrence should be triggered.
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<Vec<i32>>>,
    /// A list containing a single item which specifies the Minute interval at which this recurrence should be triggered.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<Vec<i32>>>,
    /// A list of days of the month that the job should execute on.
    #[builder(into, default)]
    #[serde(rename = "monthDays")]
    pub r#month_days: Box<Option<Vec<i32>>>,
    /// A `monthly` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Box<Option<Vec<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceScheduleMonthly>>>,
    /// A list of days of the week that the job should execute on. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into, default)]
    #[serde(rename = "weekDays")]
    pub r#week_days: Box<Option<Vec<String>>>,
}
