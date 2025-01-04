#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightsReportConfigFrequencyOptions {
    /// The date to stop generating inventory reports. For example, {"day": 15, "month": 9, "year": 2022}.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<super::super::types::storage::InsightsReportConfigFrequencyOptionsEndDate>,
    /// The frequency in which inventory reports are generated. Values are DAILY or WEEKLY.
    /// Possible values are: `DAILY`, `WEEKLY`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// The date to start generating inventory reports. For example, {"day": 15, "month": 8, "year": 2022}.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<super::super::types::storage::InsightsReportConfigFrequencyOptionsStartDate>,
}
