#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightsReportConfigFrequencyOptionsStartDate {
    /// The day of the month to start generating inventory reports.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<i32>,
    /// The month to start generating inventory reports.
    #[builder(into)]
    #[serde(rename = "month")]
    pub r#month: Box<i32>,
    /// The year to start generating inventory reports
    #[builder(into)]
    #[serde(rename = "year")]
    pub r#year: Box<i32>,
}
