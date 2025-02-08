#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RefreshScheduleScheduleScheduleFrequency {
    /// The interval between scheduled refreshes. Valid values are `MINUTE15`, `MINUTE30`, `HOURLY`, `DAILY`, `WEEKLY` and `MONTHLY`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
    /// The [refresh on entity](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ScheduleRefreshOnEntity.html) configuration for weekly or monthly schedules. See refresh_on_day.
    #[builder(into, default)]
    #[serde(rename = "refreshOnDay")]
    pub r#refresh_on_day: Box<Option<super::super::types::quicksight::RefreshScheduleScheduleScheduleFrequencyRefreshOnDay>>,
    /// The time of day that you want the dataset to refresh. This value is expressed in `HH:MM` format. This field is not required for schedules that refresh hourly.
    #[builder(into, default)]
    #[serde(rename = "timeOfTheDay")]
    pub r#time_of_the_day: Box<Option<String>>,
    /// The timezone that you want the refresh schedule to use.
    #[builder(into, default)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<Option<String>>,
}
