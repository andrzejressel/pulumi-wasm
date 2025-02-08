#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SoftwareUpdateConfigurationScheduleMonthlyOccurrence {
    /// Day of the occurrence. Must be one of `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, `Sunday`.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// Occurrence of the week within the month. Must be between `1` and `4`. `-1` for last week within the month.
    #[builder(into)]
    #[serde(rename = "occurrence")]
    pub r#occurrence: Box<i32>,
}
