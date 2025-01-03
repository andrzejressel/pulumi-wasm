#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightsReportConfigFrequencyOptionsEndDate {
    /// The day of the month to stop generating inventory reports.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<i32>,
    /// The month to stop generating inventory reports.
    #[builder(into)]
    #[serde(rename = "month")]
    pub r#month: Box<i32>,
    /// The year to stop generating inventory reports
    #[builder(into)]
    #[serde(rename = "year")]
    pub r#year: Box<i32>,
}
