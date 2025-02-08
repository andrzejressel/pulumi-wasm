#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RefreshScheduleSchedule {
    /// The type of refresh that the dataset undergoes. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
    #[builder(into)]
    #[serde(rename = "refreshType")]
    pub r#refresh_type: Box<String>,
    /// The configuration of the [schedule frequency](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshFrequency.html). See schedule_frequency.
    #[builder(into, default)]
    #[serde(rename = "scheduleFrequency")]
    pub r#schedule_frequency: Box<Option<super::super::types::quicksight::RefreshScheduleScheduleScheduleFrequency>>,
    /// Time after which the refresh schedule can be started, expressed in `YYYY-MM-DDTHH:MM:SS` format.
    #[builder(into, default)]
    #[serde(rename = "startAfterDateTime")]
    pub r#start_after_date_time: Box<Option<String>>,
}
