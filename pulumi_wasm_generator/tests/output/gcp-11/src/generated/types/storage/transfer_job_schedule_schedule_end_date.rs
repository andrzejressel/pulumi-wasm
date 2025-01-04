#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobScheduleScheduleEndDate {
    /// Day of month. Must be from 1 to 31 and valid for the year and month.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<i32>,
    /// Month of year. Must be from 1 to 12.
    #[builder(into)]
    #[serde(rename = "month")]
    pub r#month: Box<i32>,
    /// Year of date. Must be from 1 to 9999.
    #[builder(into)]
    #[serde(rename = "year")]
    pub r#year: Box<i32>,
}
