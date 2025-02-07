#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainMatchingJobSchedule {
    /// The day when the Identity Resolution Job should run every week.
    #[builder(into)]
    #[serde(rename = "dayOfTheWeek")]
    pub r#day_of_the_week: Box<String>,
    /// The time when the Identity Resolution Job should run every week.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
}
